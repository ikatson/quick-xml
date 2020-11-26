use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::prelude::AsyncBufRead;

type R<'a> = tokio::io::Result<&'a [u8]>;

pub struct FillBuf<'a, F, C> {
    inner: &'a mut F,
    callback: C,
}

pub trait FillBufExt {
    fn fill_buf<'a, C>(&'a mut self, callback: C) -> FillBuf<'a, Self, C>
    where
        Self: Sized;
}

impl<F> FillBufExt for F
where
    F: AsyncBufRead + Unpin + Sized,
{
    fn fill_buf<C>(&mut self, callback: C) -> FillBuf<'_, F, C> {
        FillBuf {
            inner: self,
            callback,
        }
    }
}

impl<'a, B: AsyncBufRead + Unpin, C: FnMut(R<'a>) + Unpin> Future for FillBuf<'a, B, C> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let z: *mut _ = self.get_mut();
        let i: &'a mut B = unsafe { (*z).inner };
        let cb = unsafe { &mut (*z).callback };
        let p = Pin::new(i);
        // todo!()
        // let s: Pin<&'a mut B> = Pin::new((*self).inner);
        match p.poll_fill_buf(cx) {
            Poll::Ready(r) => Poll::Ready((cb)(r)),
            Poll::Pending => Poll::Pending,
        }
    }
}
