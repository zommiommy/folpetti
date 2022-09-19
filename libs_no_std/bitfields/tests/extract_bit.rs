
#![cfg(test)]
use bitfields::*;

#[test]
fn test_extract_bit() {
		let r = 0b0100010111010100000000000001111011010100111010110011111101000100_u64;
		assert_eq!(r.extract_bit::<00>(), false);
		assert_eq!(r.extract_bit::<01>(), false);
		assert_eq!(r.extract_bit::<02>(), true);
		assert_eq!(r.extract_bit::<03>(), false);
		assert_eq!(r.extract_bit::<04>(), false);
		assert_eq!(r.extract_bit::<05>(), false);
		assert_eq!(r.extract_bit::<06>(), true);
		assert_eq!(r.extract_bit::<07>(), false);
		assert_eq!(r.extract_bit::<08>(), true);
		assert_eq!(r.extract_bit::<09>(), true);
		assert_eq!(r.extract_bit::<10>(), true);
		assert_eq!(r.extract_bit::<11>(), true);
		assert_eq!(r.extract_bit::<12>(), true);
		assert_eq!(r.extract_bit::<13>(), true);
		assert_eq!(r.extract_bit::<14>(), false);
		assert_eq!(r.extract_bit::<15>(), false);
		assert_eq!(r.extract_bit::<16>(), true);
		assert_eq!(r.extract_bit::<17>(), true);
		assert_eq!(r.extract_bit::<18>(), false);
		assert_eq!(r.extract_bit::<19>(), true);
		assert_eq!(r.extract_bit::<20>(), false);
		assert_eq!(r.extract_bit::<21>(), true);
		assert_eq!(r.extract_bit::<22>(), true);
		assert_eq!(r.extract_bit::<23>(), true);
		assert_eq!(r.extract_bit::<24>(), false);
		assert_eq!(r.extract_bit::<25>(), false);
		assert_eq!(r.extract_bit::<26>(), true);
		assert_eq!(r.extract_bit::<27>(), false);
		assert_eq!(r.extract_bit::<28>(), true);
		assert_eq!(r.extract_bit::<29>(), false);
		assert_eq!(r.extract_bit::<30>(), true);
		assert_eq!(r.extract_bit::<31>(), true);
		assert_eq!(r.extract_bit::<32>(), false);
		assert_eq!(r.extract_bit::<33>(), true);
		assert_eq!(r.extract_bit::<34>(), true);
		assert_eq!(r.extract_bit::<35>(), true);
		assert_eq!(r.extract_bit::<36>(), true);
		assert_eq!(r.extract_bit::<37>(), false);
		assert_eq!(r.extract_bit::<38>(), false);
		assert_eq!(r.extract_bit::<39>(), false);
		assert_eq!(r.extract_bit::<40>(), false);
		assert_eq!(r.extract_bit::<41>(), false);
		assert_eq!(r.extract_bit::<42>(), false);
		assert_eq!(r.extract_bit::<43>(), false);
		assert_eq!(r.extract_bit::<44>(), false);
		assert_eq!(r.extract_bit::<45>(), false);
		assert_eq!(r.extract_bit::<46>(), false);
		assert_eq!(r.extract_bit::<47>(), false);
		assert_eq!(r.extract_bit::<48>(), false);
		assert_eq!(r.extract_bit::<49>(), false);
		assert_eq!(r.extract_bit::<50>(), true);
		assert_eq!(r.extract_bit::<51>(), false);
		assert_eq!(r.extract_bit::<52>(), true);
		assert_eq!(r.extract_bit::<53>(), false);
		assert_eq!(r.extract_bit::<54>(), true);
		assert_eq!(r.extract_bit::<55>(), true);
		assert_eq!(r.extract_bit::<56>(), true);
		assert_eq!(r.extract_bit::<57>(), false);
		assert_eq!(r.extract_bit::<58>(), true);
		assert_eq!(r.extract_bit::<59>(), false);
		assert_eq!(r.extract_bit::<60>(), false);
		assert_eq!(r.extract_bit::<61>(), false);
		assert_eq!(r.extract_bit::<62>(), true);
		assert_eq!(r.extract_bit::<63>(), false);

}
