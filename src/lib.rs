#[derive(Debug)]
pub struct Matrix {
    dim: usize,
    data: Vec<i32>,
}

impl Matrix {
    pub fn from(dim: usize, data: Vec<i32>) -> Self {
        Self { dim, data }
    }

    pub fn random(dim: usize, cap: i32) -> Self {
        let data = (0..dim * dim)
            .map(|_| rand::random::<i32>() % cap)
            .collect();
        Self { dim, data }
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = i32;
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl Matrix {
    #[cfg(feature = "single-threaded")]
    pub fn average_pool(self, width: usize, stride: usize) -> Matrix {
        let mut data = Vec::new();
        let mut dim = 0;
        for i in (0..self.dim).step_by(stride) {
            if i + width > self.dim {
                break;
            }
            dim += 1;
            for j in (0..self.dim).step_by(stride) {
                if j + width > self.dim {
                    break;
                }
                let mut sum = 0;
                for k in 0..width {
                    for l in 0..width {
                        sum += self[(i + k, j + l)];
                    }
                }
                data.push(sum / (width * width) as i32);
            }
        }
        Self { dim, data }
    }

    #[cfg(not(feature = "single-threaded"))]
    pub fn average_pool(self, width: usize, stride: usize) -> Matrix {
        use std::thread;
        let dim = (self.dim - width) / stride + 1;
        let mut data = vec![0; dim * dim];
        let view = std::sync::Arc::new(self);

        thread::scope(|s| {
            data.chunks_mut(dim)
                .enumerate()
                .for_each(|(index, data)| {
                    let view = view.clone();
                    s.spawn(move || {
                        let i = index * stride;
                        for j in 0..dim {
                            let mut sum = 0;
                            for k in 0..width {
                                for l in 0..width {
                                    sum += view[(i + k) * view.dim + j * stride + l];
                                }
                            }
                            data[j] = sum / (width * width) as i32;
                        }
                    });
                });
        });

        Self { dim, data }
    }
}

#[test]
fn test_average_pool() {
    let m = Matrix::from(
        4,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    );
    let m = m.average_pool(2, 2);
    assert_eq!(m.dim, 2);
    assert_eq!(m.data, vec![3, 5, 11, 13]);
}
