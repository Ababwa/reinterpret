use std::{mem::{align_of, size_of}, ptr, slice};

pub unsafe fn reff<A, B>(data: &A) -> &B {
	assert!(data as *const A as usize % align_of::<B>() == 0, "input not aligned with output type");
	assert!(size_of::<A>() == size_of::<B>(), "input and output sizes do not match");
	&*(data as *const A).cast()
}

pub unsafe fn slice<A, B>(data: &[A]) -> &[B] {
	let len = data.len();
	assert!(data.as_ptr() as usize % align_of::<B>() == 0, "input not aligned with output type");
	assert!(len * size_of::<A>() % size_of::<B>() == 0, "{} items of size {} cannot be reinterpreted as any number of items of size {}", len, size_of::<A>(), size_of::<B>());
	slice::from_raw_parts(data.as_ptr().cast(), len * size_of::<A>() / size_of::<B>())
}

pub unsafe fn ref_to_slice<A, B>(data: &A) -> &[B] {
	assert!(data as *const A as usize % align_of::<B>() == 0, "input not aligned with output type");
	assert!(size_of::<A>() % size_of::<B>() == 0, "item of size {} cannot be reinterpreted as any number of items of size {}", size_of::<A>(), size_of::<B>());
	slice::from_raw_parts((data as *const A).cast(), size_of::<A>() / size_of::<B>())
}

pub unsafe fn slice_to_ref<A, B>(data: &[A]) -> &B {
	assert!(data.as_ptr() as usize % align_of::<B>() == 0, "input not aligned with output type");
	assert!(data.len() * size_of::<A>() == size_of::<B>(), "input and output sizes do not match");
	&*data.as_ptr().cast()
}

pub unsafe fn boxx<A, B>(data: Box<A>) -> Box<B> {
	assert!(data.as_ref() as *const A as usize % align_of::<B>() == 0, "input not aligned with output type");
	assert!(size_of::<A>() == size_of::<B>(), "input and output sizes do not match");
	Box::from_raw(Box::into_raw(data).cast())
}

pub unsafe fn box_slice<A, B>(data: Box<[A]>) -> Box<[B]> {
	let len = data.len();
	assert!(data.as_ptr() as usize % align_of::<B>() == 0, "input not aligned with output type");
	assert!(len * size_of::<A>() % size_of::<B>() == 0, "{} items of size {} cannot be reinterpreted as any number of items of size {}", len, size_of::<A>(), size_of::<B>());
	Box::from_raw(ptr::slice_from_raw_parts_mut(Box::into_raw(data).cast(), len * size_of::<A>() / size_of::<B>()))
}
