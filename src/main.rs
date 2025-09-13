type SectorID = u8;
struct Sector {
    id : SectorID,
    relations : Vec<SectorID>,
    is_based : bool,
    cost : f64
}

fn total_cost (g : & Vec<Sector> ) -> f64 {
    let mut s = 0.0;
    for i in g {
        if i.is_based {
            s += i.cost
        }
    };
    s
}

fn main() {
    let mut a = vec![
        Sector {
            id : ,
            relations : vec![,],
            is_based : false,
            cost : ,
        },
        Sector {
            id : ,
            relations : vec![,],
            is_based : false,
            cost : ,
        },
        Sector {
            id : ,
            relations : vec![,],
            is_based : false,
            cost : ,
        },
        Sector {
            id : ,
            relations : vec![,],
            is_based : false,
            cost : ,
        },
    ];
}
