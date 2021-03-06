use std::collections::HashMap;

fn main() {
    let (template, ruleset) = parse(include_str!("../input.txt"));
    //part 1
    let qty = polymer_synth(&template, &ruleset, 10);
    println!("Most common - least common: {}", qty);
    //part 2
    let qty = polymer_synth2(&template, &ruleset, 40);
    println!("Most common - least common after 40 generations: {}", qty);
}

type RuleSet = HashMap<[u8; 2], u8>;

fn parse(input: &str) -> (Vec<u8>, RuleSet) {
    let mut rs: RuleSet = RuleSet::new();
    let mut lines = input.lines();
    let tpl = lines.next().expect("no first line").bytes().collect();
    lines.next();
    for l in lines {
        let mut s = l.split(" -> ");
        let pair = s
            .next()
            .expect("no pair")
            .bytes()
            .collect::<Vec<u8>>()
            .try_into()
            .expect("slice convert");
        let insert = s.next().expect("no char").bytes().next().expect("no char");

        rs.insert(pair, insert);
    }
    (tpl, rs)
}
fn polymer_synth(template: &[u8], ruleset: &RuleSet, steps: usize) -> usize {
    let mut result: Vec<u8> = vec![0; template.len()];
    result.clone_from_slice(template);
    for _ in 0..steps {
        result = step(&result, ruleset);
        //println!("{}", result.iter().map(|x| *x as char).collect::<String>());
    }
    let mut freq: HashMap<u8, usize> = HashMap::new();
    for e in result.iter() {
        *freq.entry(*e).or_insert(0) += 1;
    }
    let max = freq.iter().max_by_key(|x| x.1).expect("no max").1;
    let min = freq.iter().min_by_key(|x| x.1).expect("no min").1;
    max - min
}

fn step(template: &[u8], ruleset: &RuleSet) -> Vec<u8> {
    let mut polymer = Vec::with_capacity(template.len() * 2 - 1);
    for pair in template.windows(2) {
        polymer.push(pair[0]);
        polymer.push(ruleset[pair]);
    }
    polymer.push(*template.last().expect("no last"));
    polymer
}

type Freq = HashMap<u8, usize>;
type FreqCache = HashMap<([u8; 2], usize), Freq>;

fn polymer_synth2(template: &[u8], ruleset: &RuleSet, steps: usize) -> usize {
    let mut freq = Freq::new();
    let mut fc = FreqCache::new();
    for pair in template.windows(2) {
        let p = pair.try_into().expect("not a pair");
        let f = pair_freq(p, ruleset, steps - 1, &mut fc);
        *freq.entry(p[0]).or_insert(0) += 1;
        for (k, v) in f {
            *freq.entry(k).or_insert(0) += v;
        }
    }
    *freq.entry(*template.last().expect("no last")).or_insert(0) += 1;

    let max = freq.iter().max_by_key(|x| x.1).expect("no max").1;
    let min = freq.iter().min_by_key(|x| x.1).expect("no min").1;
    max - min
}

fn pair_freq(pair: &[u8; 2], ruleset: &RuleSet, remaining: usize, fc: &mut FreqCache) -> Freq {
    if let Some(freq) = fc.get(&(*pair, remaining)) {
        return freq.clone();
    }
    let mut freq = Freq::new();

    let insert = ruleset[pair];
    *freq.entry(insert).or_insert(0) += 1;
    if remaining > 0 {
        let f1 = pair_freq(&[pair[0], insert], ruleset, remaining - 1, fc);
        let f2 = pair_freq(&[insert, pair[1]], ruleset, remaining - 1, fc);
        for (k, v) in f1.iter().chain(f2.iter()) {
            *freq.entry(*k).or_insert(0) += v;
        }
    }
    /*
    if remaining <= 2 {
        println!(
            "remaining: {} current: {}{}{} : {:?}",
            remaining, pair[0] as char, insert as char, pair[1] as char, &freq
        );
    }
    */

    let res = freq.clone();
    fc.insert((*pair, remaining), freq);

    res
}

#[test]
fn test() {
    let (template, ruleset) = parse(include_str!("../sample.txt"));
    //part 1
    let qty = polymer_synth(&template, &ruleset, 10);
    assert_eq!(qty, 1588);
    //part 2
    let qty = polymer_synth2(&template, &ruleset, 40);
    assert_eq!(qty, 2188189693529);
}
