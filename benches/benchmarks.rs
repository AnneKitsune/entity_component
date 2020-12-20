use criterion::*;
use entity_component::*;

fn create_entity_struct(c: &mut Criterion) {
    c.bench_function("Create Entity Struct", |b| {
        b.iter(|| {
            let _entities = Entities::default();
        });
    });
}

fn create_entities(c: &mut Criterion) {
    c.bench_function("Create Entities & 10000 Entity", |b| {
        b.iter(|| {
            let mut entities = Entities::default();
            for _ in 0..10000 {
                entities.create();
            }
        });
    });
}

fn create_delete_entities(c: &mut Criterion) {
    c.bench_function("Create Delete 5000 Entities 1k chunk", |b| {
        b.iter(|| {
            let mut entities = Entities::default();
            for _ in 0..5 {
                let e = (0..1000).map(|_| entities.create()).collect::<Vec<_>>();
                e.into_iter().for_each(|i| entities.kill(i));
            }
        });
    });
}

fn create_storage(c: &mut Criterion) {
    struct A;
    c.bench_function("Create Storage", |b| {
        b.iter(|| {
            let _storage = Components::<A>::default();
        });
    });
}

fn create_storage_large(c: &mut Criterion) {
    struct A([f64; 64]);
    c.bench_function("Create Storage Large", |b| {
        b.iter(|| {
            let _storage = Components::<A>::default();
        });
    });
}

fn create_with_component(c: &mut Criterion) {
    #[derive(Clone)]
    struct A;
    c.bench_function("Create 10000 With Component", |b| {
        b.iter(|| {
            let mut entities = Entities::default();
            let mut storage = Components::<A>::default();
            for _ in 0..10000 {
                let e = entities.create();
                storage.insert(e, A);
            }
        });
    });
}

fn iter_component(c: &mut Criterion) {
    struct A;
    c.bench_function("Iter 10000", |b| {
        let mut entities = Entities::default();
        let mut storage = Components::<A>::default();
        for _ in 0..10000 {
            let e = entities.create();
            storage.insert(e, A);
        }
        b.iter(|| {
            if storage.iter().count() < 4 {
                println!("no u");
            }
        });
    });
}

fn iter_mut_op(c: &mut Criterion) {
    struct A(f32);
    c.bench_function("Iter mut 10000 f32", |b| {
        let mut entities = Entities::default();
        let mut storage = Components::<A>::default();
        for _ in 0..10000 {
            let e = entities.create();
            storage.insert(e, A(1.0));
        }
        b.iter(|| {
            storage.iter_mut().for_each(|o| o.0 *= 2.0);
        });
    });
}

fn iter_mut_op_partialfill(c: &mut Criterion) {
    c.bench_function("Iter mut 10000 f32 Partial Fill", |b| {
        struct A(f32);
        let mut entities = Entities::default();
        let mut storage = Components::<A>::default();
        for i in 0..10000 {
            let e = entities.create();
            if i % 5 == 0 {
                storage.insert(e, A(1.0));
            }
        }
        b.iter(|| {
            storage.iter_mut().for_each(|o| o.0 *= 2.0);
        });
    });
}

fn join_bitset_speed(c: &mut Criterion) {
    c.bench_function("2 Join Speed", |b| {
        struct A(f32);
        struct B(f32);
        let mut entities = Entities::default();
        let mut storage = Components::<A>::default();
        let mut storage2 = Components::<B>::default();
        for _ in 0..10000 {
            let e = entities.create();
            storage.insert(e, A(1.0));
            storage2.insert(e, B(1.0));
        }
        b.iter(|| {
            join!(&mut storage && &storage2).for_each(|_| {});
        });
    });
}

fn join_iter_speed(c: &mut Criterion) {
    c.bench_function("2 Iter Speed", |b| {
        struct A(f32);
        struct B(f32);
        let mut entities = Entities::default();
        let mut storage = Components::<A>::default();
        let mut storage2 = Components::<B>::default();
        for _ in 0..10000 {
            let e = entities.create();
            storage.insert(e, A(1.0));
            storage2.insert(e, B(1.0));
        }
        b.iter(|| {
            join!(&mut storage && &storage2)
                .for_each(|(s, s2)| s.unwrap().0 += s2.unwrap().0);
        });
    });
}

fn join_immut_iter(c: &mut Criterion) {
    c.bench_function("2 Imut Iter Speed", |b| {
        struct A(f32);
        struct B(f32);
        let mut entities = Entities::default();
        let mut storage = Components::<A>::default();
        let mut storage2 = Components::<B>::default();
        for _ in 0..10000 {
            let e = entities.create();
            storage.insert(e, A(1.0));
            storage2.insert(e, B(1.0));
        }
        b.iter(|| {
            let mut count = 0;
            count += join!(&storage && &storage2).count();
            assert_eq!(count, 10000);
        });
    });
}

fn join_mut_partialfill(c: &mut Criterion) {
    c.bench_function("Join mut partial fill", |b| {
        struct A(f32);
        struct B(f32);
        let mut entities = Entities::default();
        let mut storage = Components::<A>::default();
        let mut storage2 = Components::<B>::default();
        for i in 0..10000 {
            let e = entities.create();
            if i % 5 == 0 {
                storage.insert(e, A(1.0));
            }
            if i % 6 == 0 {
                storage2.insert(e, B(1.0));
            }
        }
        b.iter(|| {
            join!(&mut storage && &storage2)
                .for_each(|(s, s2)| s.unwrap().0 += s2.unwrap().0);
        });
    });
}

criterion_group!(
    group,
    create_entity_struct,
    create_entities,
    create_delete_entities,
    create_storage,
    create_storage_large,
    create_with_component,
    iter_component,
    iter_mut_op,
    iter_mut_op_partialfill,
    join_bitset_speed,
    join_iter_speed,
    join_immut_iter,
    join_mut_partialfill,
);
criterion_main!(group);
