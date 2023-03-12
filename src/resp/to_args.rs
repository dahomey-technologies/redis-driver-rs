use crate::resp::{BulkString, CommandArgs};
use dtoa::Float;
use itoa::Integer;
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::BuildHasher,
    iter::{once, Once},
};

/// Types compatible with command args
pub trait ToArgs {
    fn write_args(&self, args: &mut CommandArgs);
    fn num_args(&self) -> usize {
        1
    }
}

fn write_integer<I: Integer>(i: I, args: &mut CommandArgs) {
    let mut buf = itoa::Buffer::new();
    let str = buf.format(i);
    args.write_arg(str.as_bytes());
}

fn write_float<F: Float>(f: F, args: &mut CommandArgs) {
    let mut buf = dtoa::Buffer::new();
    let str = buf.format(f);
    args.write_arg(str.as_bytes());
}

impl ToArgs for i8 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for i16 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for u16 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for i32 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for u32 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for i64 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for u64 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for isize {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for usize {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_integer(*self, args);
    }
}

impl ToArgs for f32 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_float(*self, args);
    }
}

impl ToArgs for f64 {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        write_float(*self, args);
    }
}

impl ToArgs for bool {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(if *self { b"1" } else { b"0" });
    }
}

impl ToArgs for BulkString {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(self.as_bytes());
    }
}

impl ToArgs for Vec<u8> {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(self.as_slice());
    }
}

impl ToArgs for &[u8] {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(self);
    }
}

impl<const N: usize> ToArgs for &[u8; N] {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(self.as_slice());
    }
}

impl<const N: usize> ToArgs for [u8; N] {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(self.as_slice());
    }
}

impl ToArgs for &str {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(self.as_bytes());
    }
}

impl ToArgs for String {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(self.as_bytes());
    }
}

impl ToArgs for &String {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        args.write_arg(self.as_bytes());
    }
}

impl ToArgs for char {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        let mut buf: [u8; 4] = [0; 4];
        self.encode_utf8(&mut buf);
        args.write_arg(buf.as_slice());
    }
}

impl<T: ToArgs> ToArgs for Option<T> {
    fn write_args(&self, args: &mut CommandArgs) {
        if let Some(t) = self {
            t.write_args(args);
        }
    }

    fn num_args(&self) -> usize {
        match self {
            Some(t) => t.num_args(),
            None => 0,
        }
    }
}

impl<T: ToArgs, const N: usize> ToArgs for [T; N] {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        for e in self {
            e.write_args(args);
        }
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.iter().fold(0, |acc, t| acc + t.num_args())
    }
}

impl<T: ToArgs> ToArgs for Vec<T> {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        for e in self {
            e.write_args(args);
        }
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.iter().fold(0, |acc, t| acc + t.num_args())
    }
}

impl<T: ToArgs> ToArgs for &[T] {
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        for e in self.iter() {
            e.write_args(args);
        }
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.iter().fold(0, |acc, t| acc + t.num_args())
    }
}

impl<T, A> ToArgs for SmallVec<A>
where
    A: smallvec::Array<Item = T>,
    T: ToArgs,
{
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        for e in self {
            e.write_args(args);
        }
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.iter().fold(0, |acc, t| acc + t.num_args())
    }
}

impl<T, S: BuildHasher> ToArgs for HashSet<T, S>
where
    T: ToArgs,
{
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        for e in self {
            e.write_args(args);
        }
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.iter().fold(0, |acc, t| acc + t.num_args())
    }
}

impl<T> ToArgs for BTreeSet<T>
where
    T: ToArgs,
{
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        for e in self {
            e.write_args(args);
        }
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.iter().fold(0, |acc, t| acc + t.num_args())
    }
}

impl<K, V, S: BuildHasher> ToArgs for HashMap<K, V, S>
where
    K: ToArgs,
    V: ToArgs,
{
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        for (key, value) in self {
            key.write_args(args);
            value.write_args(args);
        }
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.iter()
            .fold(0, |acc, (k, v)| acc + k.num_args() + v.num_args())
    }
}

impl<K, V> ToArgs for BTreeMap<K, V>
where
    K: ToArgs,
    V: ToArgs,
{
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        for (key, value) in self {
            key.write_args(args);
            value.write_args(args);
        }
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.iter()
            .fold(0, |acc, (k, v)| acc + k.num_args() + v.num_args())
    }
}

impl<T, U> ToArgs for (T, U)
where
    T: ToArgs,
    U: ToArgs,
{
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        self.0.write_args(args);
        self.1.write_args(args);
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.0.num_args() + self.1.num_args()
    }
}

impl<T, U, V> ToArgs for (T, U, V)
where
    T: ToArgs,
    U: ToArgs,
    V: ToArgs,
{
    #[inline]
    fn write_args(&self, args: &mut CommandArgs) {
        self.0.write_args(args);
        self.1.write_args(args);
        self.2.write_args(args);
    }

    #[inline]
    fn num_args(&self) -> usize {
        self.0.num_args() + self.1.num_args() + self.2.num_args()
    }
}

impl ToArgs for CommandArgs {
    fn write_args(&self, args: &mut CommandArgs) {
        for arg in self {
            args.write_arg(arg);
        }
    }
}

impl ToArgs for &CommandArgs {
    fn write_args(&self, args: &mut CommandArgs) {
        for arg in self.into_iter() {
            args.write_arg(arg);
        }
    }
}

/// Generic Marker for single arguments (no collections nor tuples)
pub trait SingleArg: ToArgs {}

impl SingleArg for i8 {}
impl SingleArg for u16 {}
impl SingleArg for i16 {}
impl SingleArg for u32 {}
impl SingleArg for i32 {}
impl SingleArg for u64 {}
impl SingleArg for i64 {}
impl SingleArg for usize {}
impl SingleArg for isize {}
impl SingleArg for f32 {}
impl SingleArg for f64 {}
impl SingleArg for bool {}
impl SingleArg for char {}
impl SingleArg for &'static str {}
impl SingleArg for String {}
impl<const N: usize> SingleArg for &[u8; N] {}
impl<const N: usize> SingleArg for [u8; N] {}
impl SingleArg for &[u8] {}
impl SingleArg for Vec<u8> {}
impl SingleArg for BulkString {}
impl<T: SingleArg> SingleArg for Option<T> {}

/// Generic Marker for Collections of `ToArgs`
///
/// Each element of the collection can produce multiple args.
pub trait MultipleArgsCollection<T>: ToArgs
where
    T: ToArgs,
{
}

impl<T, const N: usize> MultipleArgsCollection<T> for [T; N] where T: ToArgs {}
impl<T> MultipleArgsCollection<T> for Vec<T> where T: ToArgs {}
impl<T> MultipleArgsCollection<T> for T where T: ToArgs {}

/// Marker for collections of single items of `ToArgs`
///
/// Each element of the collection can only produce a single arg.
pub trait SingleArgCollection<T>: ToArgs
where
    T: SingleArg,
{
    type IntoIter: Iterator<Item = T>;

    fn into_iter(self) -> Self::IntoIter;
}

impl SingleArgCollection<Vec<u8>> for CommandArgs {
    type IntoIter = std::vec::IntoIter<Vec<u8>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

impl<T, const N: usize> SingleArgCollection<T> for [T; N]
where
    T: SingleArg,
{
    type IntoIter = std::array::IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

impl<T> SingleArgCollection<T> for Vec<T>
where
    T: SingleArg,
{
    type IntoIter = std::vec::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

impl<A, T> SingleArgCollection<T> for SmallVec<A>
where
    A: smallvec::Array<Item = T>,
    T: SingleArg,
{
    type IntoIter = smallvec::IntoIter<A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

impl<T, S: BuildHasher> SingleArgCollection<T> for HashSet<T, S>
where
    T: SingleArg,
{
    type IntoIter = std::collections::hash_set::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

impl<T> SingleArgCollection<T> for BTreeSet<T>
where
    T: SingleArg,
{
    type IntoIter = std::collections::btree_set::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

impl<T> SingleArgCollection<T> for T
where
    T: SingleArg,
{
    type IntoIter = Once<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        once(self)
    }
}

/// Marker for key/value collections of Args
///
/// The key and the value can only produce a single arg each.
pub trait KeyValueArgsCollection<K, V>: ToArgs
where
    K: SingleArg,
    V: SingleArg,
{
}

impl<K, V> KeyValueArgsCollection<K, V> for Vec<(K, V)>
where
    K: SingleArg,
    V: SingleArg,
{
}

impl<A, K, V> KeyValueArgsCollection<K, V> for SmallVec<A>
where
    A: smallvec::Array<Item = (K, V)>,
    K: SingleArg,
    V: SingleArg,
{
}

impl<K, V, const N: usize> KeyValueArgsCollection<K, V> for [(K, V); N]
where
    K: SingleArg,
    V: SingleArg,
{
}

impl<K, V> KeyValueArgsCollection<K, V> for (K, V)
where
    K: SingleArg,
    V: SingleArg,
{
}

impl<K, V, S: BuildHasher> KeyValueArgsCollection<K, V> for HashMap<K, V, S>
where
    K: SingleArg,
    V: SingleArg,
{
}

impl<K, V> KeyValueArgsCollection<K, V> for BTreeMap<K, V>
where
    K: SingleArg,
    V: SingleArg,
{
}
