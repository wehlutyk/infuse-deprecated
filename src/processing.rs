use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use iron::typemap::Key;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;

use models::{Job, Document, NewDocument};
use utils::Pool;


const MAX_WORKERS: i32 = 4;

#[derive(Debug)]
enum JobError {
    Diesel(DieselError),
    GrobidError,
}

#[derive(Debug)]
enum Event {
    New,
    Finish(Result<(), JobError>),
}

#[derive(Debug)]
pub struct Message {
    event: Event,
    job_id: i32,
}

impl Message {
    pub fn new_job(id: i32) -> Message {
        Message { event: Event::New, job_id: id }
    }

    fn finish_job(id: i32, result: Result<(), JobError>) -> Message {
        Message { event: Event::Finish(result), job_id: id }
    }
}

pub type ProcessorSender = Sender<Message>;
pub type ProcessorReceiver = Receiver<Message>;
pub type ProcessorChannel = (ProcessorSender, ProcessorReceiver);

pub struct Processor {
    sender: ProcessorSender,
    receiver: ProcessorReceiver,
    pool: Pool,
}

impl Processor {
    pub fn new(channel: ProcessorChannel, pool: Pool) -> Processor {
        Processor { sender: channel.0, receiver: channel.1, pool: pool }
    }

    pub fn start(self) {
        use schema::jobs;

        info!("starting parallel processor");

        // Set all jobs to stopped.
        let connection = self.pool.get().unwrap();
        let running_jobs = jobs::table.filter(jobs::columns::running.eq(true));
        let running_count = running_jobs.count().first::<i64>(&*connection).unwrap();
        if running_count > 0 {
            info!("resetting {} jobs to non-running state", running_count);
            diesel::update(running_jobs)
                .set(jobs::columns::running.eq(false))
                .execute(&*connection).unwrap();
        }

        // Start as many jobs as we can.
        let init_job_ids = jobs::table
            .filter(jobs::columns::document_id.is_null())
            .order(jobs::columns::id.asc())
            .limit(MAX_WORKERS as i64)
            .select(jobs::columns::id)
            .load::<i32>(&*connection).unwrap();
        for job_id in init_job_ids {
            self.sender.send(Message::new_job(job_id)).unwrap();
        }

        thread::spawn(move || {
            self.event_loop();
        });
    }

    fn event_loop(self) {
        info!("starting event loop");

        let mut active_workers = 0;
        for msg in self.receiver.iter() {
            let new_job = match msg.event {
                Event::New => {
                    info!("new job queued: {}", msg.job_id);
                    if active_workers < MAX_WORKERS {
                        Some(msg.job_id)
                    } else {
                        None
                    }
                }
                Event::Finish(result) => {
                    match result {
                        Ok(_) => info!("job completed: {}", msg.job_id),
                        Err(e) => info!("job errored with {:?}: {}", e, msg.job_id),
                    }
                    active_workers -= 1;
                    self.next_job_id()
                }
            };

            if let Some(id) = new_job {
                info!("starting job {}", id);
                active_workers += 1;
                self.start_job(id);
            }
        }
    }

    fn next_job_id(&self) -> Option<i32> {
        use schema::jobs;

        let connection = self.pool.get().unwrap();
        match jobs::table
                .filter(jobs::columns::document_id.is_null())
                .filter(jobs::columns::running.eq(false))
                .order(jobs::columns::id.asc())
                .select(jobs::columns::id)
                .first::<i32>(&*connection) {
            Ok(id) => Some(id),
            Err(DieselError::NotFound) => None,
            Err(err) => panic!(err),
        }
    }

    fn start_job(&self, id: i32) {
        use schema::{jobs, documents};

        let connection = self.pool.get().unwrap();
        let sender = self.sender.clone();

        let job = match diesel::update(jobs::table.find(id))
                .set(jobs::columns::running.eq(true))
                .get_result::<Job>(&*connection) {
            Ok(job) => job,
            Err(error) => {
                sender.send(Message::finish_job(id, Err(JobError::Diesel(error)))).unwrap();
                return;
            }
        };

        thread::spawn(move || {
            // do the actual http handling
            // - file:///home/sl/Code/infuse/target/doc/multipart/client/lazy/struct.Multipart.html
            // - file:///home/sl/Code/infuse/target/doc/hyper/client/struct.Client.html
            thread::sleep(Duration::new(5, 0));
            let tei = format!("Came from job {}, hash {}", job.id, job.sha);

            // TODO: check DOI uniqueness.

            let new_document = NewDocument { tei: &tei };
            let document = match diesel::insert(&new_document)
                    .into(documents::table)
                    .get_result::<Document>(&*connection) {
                Ok(document) => document,
                Err(error) => {
                    sender.send(Message::finish_job(id, Err(JobError::Diesel(error)))).unwrap();
                    return;
                }
            };

            match diesel::update(jobs::table.find(id))
                    .set((jobs::columns::running.eq(false), jobs::columns::document_id.eq(document.id)))
                    .execute(&*connection) {
                Ok(_) => sender.send(Message::finish_job(id, Ok(()))).unwrap(),
                Err(error) => {
                    sender.send(Message::finish_job(id, Err(JobError::Diesel(error)))).unwrap();
                    return;
                }
            };
        });
    }
}

impl Key for Processor {
    type Value = Sender<Message>;
}
