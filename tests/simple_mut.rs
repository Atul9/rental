#[macro_use]
extern crate rental;


pub struct Foo {
	i: i32,
}

impl Foo {
	fn try_borrow_mut(&mut self) -> Result<&mut i32, ()> { Ok(&mut self.i) }
	fn fail_borrow_mut(&mut self) -> Result<&mut i32, ()> { Err(()) }
}


rental! {
	mod rentals {
		use super::*;

		#[rental_mut(deref_mut_suffix)]
		pub struct SimpleMut {
			foo: Box<Foo>,
			iref: &'foo mut i32,
		}
	}
}


#[test]
fn new() {
	let foo = Foo { i: 5 };
	let sm = rentals::SimpleMut::new(Box::new(foo), |foo| &mut foo.i);

	let foo = Foo { i: 5 };
	let sm: rental::TryNewResult<_, (), _> = rentals::SimpleMut::try_new(Box::new(foo), |foo| foo.try_borrow_mut());
	assert!(sm.is_ok());

	let foo = Foo { i: 5 };
	let sm: rental::TryNewResult<_, (), _> = rentals::SimpleMut::try_new(Box::new(foo), |foo| foo.fail_borrow_mut());
	assert!(sm.is_err());
}


#[test]
fn read() {
	let foo = Foo { i: 5 };

	let sm = rentals::SimpleMut::new(Box::new(foo), |foo| &mut foo.i);
	let i: i32 = sm.rent(|iref| **iref);
	assert_eq!(i, 5);

	let iref: &i32 = sm.ref_rent(|iref| *iref);
	assert_eq!(*iref, 5);

	assert_eq!(*sm, 5);
}


#[test]
fn write() {
	let foo = Foo { i: 5 };

	let mut sm = rentals::SimpleMut::new(Box::new(foo), |foo| &mut foo.i);

	{
		let iref: &mut i32 = sm.ref_rent_mut(|iref| *iref);
		*iref = 12;
		assert_eq!(*iref, 12);
	}

	*sm = 47;
	assert_eq!(*sm, 47);
}
