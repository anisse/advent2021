fn main() {
    let chiton_map = parse(include_str!("../input.txt"));
    //part 1
    let risk_level = shortest_path(&chiton_map);
    println!("Shortest path risk level: {}", risk_level);
    //part 2
    let risk_level_big = shortest_path25(&chiton_map);
    println!("Big map shortest path risk level: {}", risk_level_big);
}
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|x| x - b'0').collect())
        .collect()
}
fn shortest_path(map: &[Vec<u8>]) -> usize {
    let mut minmap = vec![vec![usize::MAX; map[0].len()]; map.len()];
    //let mut tmp = Vec::new();
    shortest_path_recur(map, 0, &mut minmap, 20, 0, 0);
    let end_x = map[0].len() - 1;
    let end_y = map.len() - 1;
    /*
    println!(
        "{}",
        minmap
            .iter()
            .map(|l| l.iter().map(|p| format!("{:3} ", p)).collect::<String>() + "\n")
            .collect::<String>()
    );
    */
    minmap[end_y][end_x]
}
fn shortest_path_recur(
    map: &[Vec<u8>],
    total: usize,
    minmap: &mut Vec<Vec<usize>>,
    backbudget: usize,
    x: usize,
    y: usize,
) {
    let end_x = map[0].len() - 1;
    let end_y = map.len() - 1;
    if total >= minmap[y][x] {
        return;
    }
    minmap[y][x] = total;
    if x == end_x && y == end_y {
        return;
    }
    for i in 0..4 {
        let (x2, y2, back2) = match i {
            0 if x < end_x => (x + 1, y, backbudget),
            1 if y < end_y => (x, y + 1, backbudget),
            2 if backbudget > 0 && x > 0 => (x - 1, y, backbudget - 1),
            3 if backbudget > 0 && y > 0 => (x, y - 1, backbudget - 1),
            _ => continue,
        };
        shortest_path_recur(map, total + map[y2][x2] as usize, minmap, back2, x2, y2);
    }
}
fn bigger_map(map: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let size_x = map[0].len();
    let size_y = map.len();
    let mut bigmap: Vec<Vec<u8>> = vec![vec![0; size_x * 5]; size_y * 5];
    // Build 5 times bigger map
    bigmap.iter_mut().enumerate().for_each(move |(y, l)| {
        l.iter_mut().enumerate().for_each(move |(x, r)| {
            let diff = (y / size_y) + (x / size_x);
            /*
            dbg!(x / size_x);
            dbg!(y / size_y);
            dbg!(diff);
            */
            *r = ((map[y % size_y][x % size_x] as usize + diff) % 10) as u8;
            if *r < map[y % size_y][x % size_x] {
                *r += 1;
            }
        })
    });
    //print_map(&bigmap);
    bigmap
}
fn shortest_path25(map: &[Vec<u8>]) -> usize {
    let bigmap = bigger_map(map);
    shortest_path(&bigmap)
}

fn print_map(map: &[Vec<u8>]) {
    println!(
        "{}",
        map.iter()
            .map(|l| l.iter().map(|p| format!("{} ", p)).collect::<String>() + "\n")
            .collect::<String>()
    );
}

#[test]
fn test() {
    let chiton_map = parse(include_str!("../sample.txt"));
    //part 1
    let risk_level = shortest_path(&chiton_map);
    assert_eq!(risk_level, 40);
    //part 2
    let risk_level = shortest_path25(&chiton_map);
    assert_eq!(risk_level, 315);

    let chiton_map = parse(include_str!("../backtracking.txt"));
    let risk_level = shortest_path(&chiton_map);
    assert_eq!(risk_level, 8);
}

#[test]
fn test_bigger() {
    assert_eq!(
        bigger_map(&[vec![8_u8]]),
        vec![
            vec![8, 9, 1, 2, 3],
            vec![9, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
        ]
    );
    let chiton_map = parse(include_str!("../sample.txt"));
    assert_eq!(
        bigger_map(&chiton_map),
        parse(
            "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479
"
        )
    );
}
