use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use num_complex::Complex;

const NEIGHBORS_4: [Point; 4] = [
    Point(Complex::new(1, 0)),
    Point(Complex::new(-1, 0)),
    Point(Complex::new(0, 1)),
    Point(Complex::new(0, -1)),
];

const NEIGHBORS_8: [Point; 8] = [
    Point(Complex::new(1, 0)),
    Point(Complex::new(-1, 0)),
    Point(Complex::new(0, 1)),
    Point(Complex::new(0, -1)),
    Point(Complex::new(1, 1)),
    Point(Complex::new(1, -1)),
    Point(Complex::new(1, 1)),
    Point(Complex::new(-1, 1)),
];

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    derive_more::Add,
    derive_more::Sub,
    derive_more::Sum,
    derive_more::AddAssign,
    derive_more::SubAssign,
    derive_more::From,
    derive_more::Into,
)]
pub struct Point(Complex<isize>);

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self(Complex::new(x, y))
    }

    pub fn x(self) -> isize {
        self.0.re
    }

    pub fn y(self) -> isize {
        self.0.im
    }

    pub fn row(self) -> isize {
        self.y()
    }

    pub fn col(self) -> isize {
        self.x()
    }

    pub fn neighbors4(self) -> impl ExactSizeIterator<Item = Self> {
        NEIGHBORS_4.iter().copied().map(move |i| i + self)
    }

    pub fn neighbors8(self) -> impl ExactSizeIterator<Item = Self> {
        NEIGHBORS_8.iter().copied().map(move |i| i + self)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0.re, self.0.im)
    }
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    derive_more::From,
    derive_more::Into,
    derive_more::IntoIterator,
    derive_more::Index,
    derive_more::IndexMut,
)]
pub struct SparseGrid<T>(HashMap<Point, T>);

impl<T> FromIterator<(Point, T)> for SparseGrid<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Point, T)>,
    {
        Self(iter.into_iter().collect())
    }
}

impl<T> SparseGrid<T> {
    pub fn contains(&self, point: &Point) -> bool {
        self.0.contains_key(point)
    }

    pub fn insert(&mut self, point: Point, value: T) -> Option<T> {
        self.0.insert(point, value)
    }

    pub fn remove(&mut self, point: &Point) -> Option<T> {
        self.0.remove(point)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (Point, &T)> {
        self.0.iter().map(|(i, v)| (*i, v))
    }

    pub fn neighbors4(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        point
            .neighbors4()
            .filter(|point| self.contains(point))
            .map(|p| (p, &self[&p]))
    }

    pub fn neighbors8(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        point
            .neighbors8()
            .filter(|point| self.contains(point))
            .map(|p| (p, &self[&p]))
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    derive_more::IntoIterator,
    derive_more::Index,
    derive_more::IndexMut,
)]
pub struct Grid<T> {
    #[into_iterator(owned, ref, ref_mut)]
    #[index]
    #[index_mut]
    points: Vec<T>,
    n_rows: usize,
    n_cols: usize,
}

impl<T> Grid<T> {
    pub fn new(points: Vec<T>, n_rows: usize, n_cols: usize) -> Self {
        assert_eq!(n_rows * n_cols, points.len());
        Self {
            points,
            n_rows,
            n_cols,
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Get 1-d index from a point
    pub fn index_from_point(&self, point: Point) -> usize {
        usize::try_from(point.y()).unwrap() * self.n_cols + usize::try_from(point.x()).unwrap()
    }

    /// Get 2-d index from 1-d index
    pub fn point_from_index(&self, i: usize) -> Point {
        Point::new(
            isize::try_from(i % self.n_cols).unwrap(),
            isize::try_from(i / self.n_cols).unwrap(),
        )
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (Point, &T)> + ExactSizeIterator {
        self.points
            .iter()
            .enumerate()
            .map(|(i, v)| (self.point_from_index(i), v))
    }

    pub fn iter_row(
        &self,
        row: usize,
    ) -> impl DoubleEndedIterator<Item = (Point, &T)> + ExactSizeIterator {
        assert!(row < self.n_rows);
        self.points
            .iter()
            .enumerate()
            .skip(row * self.n_cols)
            .take(self.n_cols)
            .map(|(i, v)| (self.point_from_index(i), v))
    }

    pub fn iter_col(
        &self,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = (Point, &T)> + ExactSizeIterator {
        assert!(col < self.n_cols);
        self.points
            .iter()
            .enumerate()
            .skip(col)
            .step_by(self.n_cols)
            .map(|(i, v)| (self.point_from_index(i), v))
    }

    pub fn neighbors4(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        point
            .neighbors4()
            .filter(|point| {
                point.x() >= 0
                    && point.x() < isize::try_from(self.n_cols).unwrap()
                    && point.y() >= 0
                    && point.y() < isize::try_from(self.n_rows).unwrap()
            })
            .map(|p| (p, &self[p]))
    }

    pub fn neighbors8(&self, point: Point) -> impl Iterator<Item = (Point, &T)> {
        point
            .neighbors8()
            .filter(|point| {
                point.x() >= 0
                    && point.x() < isize::try_from(self.n_cols).unwrap()
                    && point.y() >= 0
                    && point.y() < isize::try_from(self.n_rows).unwrap()
            })
            .map(|p| (p, &self[p]))
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        let index = self.index_from_point(point);
        &self.points[index]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let index = self.index_from_point(point);
        &mut self.points[index]
    }
}
