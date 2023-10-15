use mastic::collect::*;
use mastic::prg;
use mastic::*;
use rayon::prelude::*;
// use prio::field::Field64;

#[test]
fn collect_test_eval_groups() {
    let client_strings = [
        "abdef", "abdef", "abdef", "ghijk", "gZijk", "gZ???", "  ?*g", "abdef", "gZ???", "gZ???",
    ];

    let strlen = crate::string_to_bits(&client_strings[0]).len();

    let seed = prg::PrgSeed::random();
    let mut col_0 = KeyCollection::new(0, &seed, strlen);
    let mut col_1 = KeyCollection::new(1, &seed, strlen);

    for cstr in &client_strings {
        let (keys_0, keys_1) = dpf::DPFKey::<u64, u64>::gen_from_str(&cstr);
        col_0.add_key(keys_0);
        col_1.add_key(keys_1);
    }

    col_0.tree_init();
    col_1.tree_init();

    let threshold = 2u64;
    let malicious = Vec::<usize>::new();
    for level in 0..strlen - 1 {
        println!("At level {:?}", level);
        let (cnt_values_0, _, _) = col_0.tree_crawl(1usize, &malicious, false);
        let (cnt_values_1, _, _) = col_1.tree_crawl(1usize, &malicious, false);

        assert_eq!(cnt_values_0.len(), cnt_values_1.len());
        let keep = KeyCollection::<u64>::keep_values(&threshold, &cnt_values_0, &cnt_values_1);

        col_0.tree_prune(&keep);
        col_1.tree_prune(&keep);
    }

    let (cnt_values_0, hashes_0) = col_0.tree_crawl_last();
    let (cnt_values_1, hashes_1) = col_1.tree_crawl_last();

    assert_eq!(cnt_values_0.len(), cnt_values_1.len());
    assert_eq!(hashes_0.len(), hashes_1.len());

    let verified = hashes_0
        .par_iter()
        .zip(hashes_1.par_iter())
        .all(|(&h0, &h1)| h0 == h1);
    assert!(verified);

    let keep = KeyCollection::<u64>::keep_values(&threshold, &cnt_values_0, &cnt_values_1);

    col_0.tree_prune(&keep);
    col_1.tree_prune(&keep);

    let shares_0 = col_0.final_shares();
    let shares_1 = col_1.final_shares();

    for res in &KeyCollection::<u64>::final_values(&shares_0, &shares_1) {
        println!("Path = {:?}", res.path);
        let s = crate::bits_to_string(&res.path);
        println!("fast: {:?} = {:?}", s, res.value);

        match &s[..] {
            "abdef" => assert_eq!(res.value, 4u64),
            "gZ???" => assert_eq!(res.value, 3u64),
            _ => {
                println!("Unexpected string: '{:?}' = {:?}", s, res.value);
                assert!(false);
            }
        }
    }
}

#[test]
fn collect_test_eval_full_groups() {
    let client_strings = [
        "01234567012345670123456701234567",
        "z12x45670y2345670123456701234567",
        "612x45670y2345670123456701234567",
        "912x45670y2345670123456701234567",
    ];

    let num_clients = 10;
    let strlen = crate::string_to_bits(&client_strings[0]).len();

    let seed = prg::PrgSeed::random();
    let mut col_0 = KeyCollection::new(0, &seed, strlen);
    let mut col_1 = KeyCollection::new(1, &seed, strlen);

    let mut keys = vec![];
    println!("Starting to generate keys");
    for s in &client_strings {
        keys.push(dpf::DPFKey::<u64, u64>::gen_from_str(&s));
    }
    println!("Done generating keys");

    for i in 0..num_clients {
        let copy_0 = keys[i % keys.len()].0.clone();
        let copy_1 = keys[i % keys.len()].1.clone();
        col_0.add_key(copy_0);
        col_1.add_key(copy_1);
        if i % 50 == 0 {
            println!("  Key {:?}", i);
        }
    }

    col_0.tree_init();
    col_1.tree_init();

    let threshold = 2u64;
    let malicious = Vec::<usize>::new();
    for level in 0..strlen - 1 {
        println!("...start");
        let (cnt_values_0, _, _) = col_0.tree_crawl(1usize, &malicious, false);
        let (cnt_values_1, _, _) = col_1.tree_crawl(1usize, &malicious, false);
        println!("...done");
        println!("At level {:?} (size: {:?})", level, cnt_values_0.len());

        assert_eq!(cnt_values_0.len(), cnt_values_1.len());
        let keep = KeyCollection::<u64>::keep_values(&threshold, &cnt_values_0, &cnt_values_1);

        col_0.tree_prune(&keep);
        col_1.tree_prune(&keep);
    }

    let (cnt_values_0, hashes_0) = col_0.tree_crawl_last();
    let (cnt_values_1, hashes_1) = col_1.tree_crawl_last();

    assert_eq!(cnt_values_0.len(), cnt_values_1.len());
    assert_eq!(hashes_0.len(), hashes_1.len());

    let verified = hashes_0
        .par_iter()
        .zip(hashes_1.par_iter())
        .all(|(&h0, &h1)| h0 == h1);
    assert!(verified);

    let keep = KeyCollection::<u64>::keep_values(&threshold, &cnt_values_0, &cnt_values_1);

    col_0.tree_prune(&keep);
    col_1.tree_prune(&keep);

    let s0 = col_0.final_shares();
    let s1 = col_1.final_shares();

    for res in &KeyCollection::<u64>::final_values(&s0, &s1) {
        println!("Path = {:?}", res.path);
        let s = crate::bits_to_string(&res.path);
        println!("Value: {:?} = {:?}", s, res.value);
    }
}

#[test]
fn collect_test_eval_fields() {
    let client_strings = [
        "abdef", "abdef", "abdef", "ghijk", "gZijk", "gZ???", "  ?*g", "abdef", "gZ???", "gZ???",
    ];

    let strlen = crate::string_to_bits(&client_strings[0]).len();

    let seed = prg::PrgSeed::random();
    let mut col_0 = KeyCollection::new(0, &seed, strlen);
    let mut col_1 = KeyCollection::new(1, &seed, strlen);

    for cstr in &client_strings {
        let (keys_0, keys_1) = dpf::DPFKey::<FieldElm, FieldElm>::gen_from_str(&cstr);
        col_0.add_key(keys_0);
        col_1.add_key(keys_1);
    }

    col_0.tree_init();
    col_1.tree_init();

    let threshold = FieldElm::from(2);
    let malicious = Vec::<usize>::new();
    for level in 0..strlen - 1 {
        println!("At level {:?}", level);
        let (cnt_values_0, _, _) = col_0.tree_crawl(1usize, &malicious, false);
        let (cnt_values_1, _, _) = col_1.tree_crawl(1usize, &malicious, false);

        assert_eq!(cnt_values_0.len(), cnt_values_1.len());
        let keep = KeyCollection::<FieldElm>::keep_values(&threshold, &cnt_values_0, &cnt_values_1);

        col_0.tree_prune(&keep);
        col_1.tree_prune(&keep);
    }

    let (cnt_values_0, hashes_0) = col_0.tree_crawl_last();
    let (cnt_values_1, hashes_1) = col_1.tree_crawl_last();

    assert_eq!(cnt_values_0.len(), cnt_values_1.len());
    assert_eq!(hashes_0.len(), hashes_1.len());

    let verified = hashes_0
        .par_iter()
        .zip(hashes_1.par_iter())
        .all(|(&h0, &h1)| h0 == h1);
    assert!(verified);

    let keep = KeyCollection::<FieldElm>::keep_values(&threshold, &cnt_values_0, &cnt_values_1);

    col_0.tree_prune(&keep);
    col_1.tree_prune(&keep);

    let shares_0 = col_0.final_shares();
    let shares_1 = col_1.final_shares();

    for res in &KeyCollection::<FieldElm>::final_values(&shares_0, &shares_1) {
        println!("Path = {:?}", res.path);
        let s = crate::bits_to_string(&res.path);
        println!("fast: {:?} = {:?}", s, res.value);

        match &s[..] {
            "abdef" => assert_eq!(res.value, FieldElm::from(4)),
            "gZ???" => assert_eq!(res.value, FieldElm::from(3)),
            _ => {
                println!("Unexpected string: '{:?}' = {:?}", s, res.value);
                assert!(false);
            }
        }
    }
}

#[test]
fn collect_test_eval_full() {
    let client_strings = [
        "01234567012345670123456701234567",
        "z12x45670y2345670123456701234567",
        "612x45670y2345670123456701234567",
        "912x45670y2345670123456701234567",
    ];

    let num_clients = 10;
    let strlen = crate::string_to_bits(&client_strings[0]).len();

    let seed = prg::PrgSeed::random();
    let mut col_0 = KeyCollection::new(0, &seed, strlen);
    let mut col_1 = KeyCollection::new(1, &seed, strlen);

    let mut keys = vec![];
    println!("Starting to generate keys");
    for s in &client_strings {
        keys.push(dpf::DPFKey::<FieldElm, FieldElm>::gen_from_str(
            &s,
        ));
    }
    println!("Done generating keys");

    for i in 0..num_clients {
        let copy_0 = keys[i % keys.len()].0.clone();
        let copy_1 = keys[i % keys.len()].1.clone();
        col_0.add_key(copy_0);
        col_1.add_key(copy_1);
        if i % 50 == 0 {
            println!("  Key {:?}", i);
        }
    }

    col_0.tree_init();
    col_1.tree_init();

    let threshold = FieldElm::from(2);
    let malicious = Vec::<usize>::new();
    for level in 0..strlen - 1 {
        println!("...start");
        let (cnt_values_0, _, _) = col_0.tree_crawl(1usize, &malicious, false);
        let (cnt_values_1, _, _) = col_1.tree_crawl(1usize, &malicious, false);
        println!("...done");
        println!("At level {:?} (size: {:?})", level, cnt_values_0.len());

        assert_eq!(cnt_values_0.len(), cnt_values_1.len());
        let keep =
            KeyCollection::<FieldElm>::keep_values(&threshold, &cnt_values_0, &cnt_values_1);

        col_0.tree_prune(&keep);
        col_1.tree_prune(&keep);
    }

    let (cnt_values_0, hashes_0) = col_0.tree_crawl_last();
    let (cnt_values_1, hashes_1) = col_1.tree_crawl_last();

    assert_eq!(cnt_values_0.len(), cnt_values_1.len());
    assert_eq!(hashes_0.len(), hashes_1.len());

    let verified = hashes_0
        .par_iter()
        .zip(hashes_1.par_iter())
        .all(|(&h0, &h1)| h0 == h1);
    assert!(verified);

    let keep =
        KeyCollection::<FieldElm>::keep_values(&threshold, &cnt_values_0, &cnt_values_1);

    col_0.tree_prune(&keep);
    col_1.tree_prune(&keep);

    let s0 = col_0.final_shares();
    let s1 = col_1.final_shares();

    for res in &KeyCollection::<FieldElm>::final_values(&s0, &s1) {
        println!("Path = {:?}", res.path);
        let s = crate::bits_to_string(&res.path);
        println!("Value: {:?} = {:?}", s, res.value);
    }
}
