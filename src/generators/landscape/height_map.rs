pub fn arrange() {
    diesel::select()
}

fn colibrate_tiles(some_tiles: Vec<Vec<Vec<f64>>>) -> Vec<Vec<Vec<f64>>> {
    let mut tiles = some_tiles;

    for irow in 0..tiles.len() {
        for itile in 0..tiles[irow].len() {
            for krow in 0..tiles.len() {
                for ktile in 0..tiles[krow].len() {
                    if tiles[irow][itile][0] == tiles[krow][ktile][0]
                        && tiles[irow][itile][1] != tiles[krow][ktile][1]
                        && tiles[irow][itile][2] == tiles[krow][ktile][2]
                    {
                        let y = (tiles[irow][itile][1] + tiles[krow][ktile][1]) / 2.0;

                        tiles[irow][itile][1] = y.clone();
                        tiles[krow][ktile][1] = y.clone();
                    }
                }
            }
        }
    }

    return tiles;
}
