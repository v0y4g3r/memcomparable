// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use memcomparable::Serializer;
use serde::Serializer as _;

criterion_group!(benches, decimal, bytes, string_io);
criterion_main!(benches);

#[cfg(not(feature = "decimal"))]
fn decimal(_c: &mut Criterion) {}

fn string_io(c: &mut Criterion) {
    use memcomparable::Deserializer;
    use serde::{Deserialize, Serialize};

    for len in [8, 24, 64, 256] {
        let s = "x".repeat(len);
        let encoded = memcomparable::to_vec(&s).unwrap();

        c.bench_function(format!("serialize_string/{len}").as_str(), |b| {
            b.iter(|| {
                let mut ser = Serializer::new(vec![]);
                s.serialize(&mut ser).unwrap();
                ser.into_inner()
            })
        });

        c.bench_function(format!("deserialize_string/{len}").as_str(), |b| {
            b.iter(|| String::deserialize(&mut Deserializer::new(encoded.as_slice())).unwrap())
        });

        c.bench_function(format!("read_bytes_into/{len}").as_str(), |b| {
            let mut buffer = Vec::new();
            b.iter(|| {
                Deserializer::new(encoded.as_slice())
                    .read_bytes_into(&mut buffer)
                    .unwrap()
            })
        });
    }
}

fn bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes");

    for size in [10, 100, 1000] {
        let bytes = (0..size).map(|_| rand::random::<u8>()).collect::<Vec<_>>();
        group.bench_function(format!("size-{}", size), |b| {
            b.iter(|| {
                let mut s = Serializer::new(Vec::with_capacity(size / 8 * 9));
                s.serialize_bytes(&bytes).unwrap();
                black_box(s);
            });
        });

        group.bench_function(format!("size-{}-reverse", size), |b| {
            b.iter(|| {
                let mut s = Serializer::new(Vec::with_capacity(size / 8 * 9));
                s.set_reverse(true);
                s.serialize_bytes(&bytes).unwrap();
                black_box(s);
            });
        });
    }
    group.finish();
}

#[cfg(feature = "decimal")]
fn decimal(c: &mut Criterion) {
    use memcomparable::{Decimal, Deserializer, Serializer};

    // generate decimals
    let mut decimals = vec![];
    for _ in 0..10 {
        decimals.push(Decimal::Normalized(rand::random()));
    }

    c.bench_function("serialize_decimal", |b| {
        let mut i = 0;
        b.iter(|| {
            let mut ser = Serializer::new(vec![]);
            ser.serialize_decimal(decimals[i]).unwrap();
            i += 1;
            if i == decimals.len() {
                i = 0;
            }
        })
    });

    c.bench_function("deserialize_decimal", |b| {
        let encodings = decimals
            .iter()
            .map(|d| {
                let mut ser = Serializer::new(vec![]);
                ser.serialize_decimal(*d).unwrap();
                ser.into_inner()
            })
            .collect::<Vec<_>>();
        let mut i = 0;
        b.iter(|| {
            Deserializer::new(encodings[i].as_slice())
                .deserialize_decimal()
                .unwrap();
            i += 1;
            if i == decimals.len() {
                i = 0;
            }
        })
    });
}
