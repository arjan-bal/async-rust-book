use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum Poll<T> {
    Ready(T),
    Pending,
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

struct Socket {}

impl Socket {
    fn has_data_to_read(&self) -> bool {
        todo!()
    }

    fn read_buf(&self) -> Vec<u8> {
        todo!()
    }

    fn set_readable_callback(&self, callback: fn()) {}
}

struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            return Poll::Ready(self.socket.read_buf());
        }
        self.socket.set_readable_callback(wake);
        Poll::Pending
    }
}

struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            return Poll::Ready(());
        }
        Poll::Pending
    }
}

struct AndThenFut<FutureFirst, FutureSecond> {
    first: Option<FutureFirst>,
    second: FutureSecond,
}

impl<FutureFirst, FutureSecond> SimpleFuture for AndThenFut<FutureFirst, FutureSecond>
where
    FutureFirst: SimpleFuture<Output = ()>,
    FutureSecond: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            if let Poll::Pending = first.poll(wake) {
                return Poll::Pending;
            }
            self.first.take();
        }
        self.second.poll(wake)
    }
}

fn main() {
    let future = hello_world();
    block_on(future)
}
