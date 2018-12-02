use std::io;
use futures::io::{ AsyncRead, AsyncWrite };
use futures::task::{ LocalWaker, Poll };


macro_rules! a {
    ( > $r:expr ) => {
        match $r {
            Poll::Ready(Ok(n)) => Ok(n),
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
            Poll::Ready(Err(e)) => Err(e)
        }
    };
}

pub struct TaskStream<'a, S: 'a> {
    pub io: &'a mut S,
    pub task: &'a LocalWaker
}

impl<'a, S> io::Read for TaskStream<'a, S>
    where S: AsyncRead + AsyncWrite
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        a!(> self.io.poll_read(self.task, buf))
    }
}

impl<'a, S> io::Write for TaskStream<'a, S>
    where S: AsyncRead + AsyncWrite
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        a!(> self.io.poll_write(self.task, buf))
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        a!(> self.io.poll_flush(self.task))
    }
}
