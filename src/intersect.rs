use geo_types::{CoordFloat, CoordNum, Coordinate, LineString};

pub struct IntersectionResult<N: CoordNum> {
    pub u: N,
    pub t: N,
    pub point: Coordinate<N>,
    pub index: usize,
}

fn cross_product<N>(a: Coordinate<N>, b: Coordinate<N>) -> N
where
    N: CoordNum,
{
    a.x * b.y - a.y * b.x
}

// https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/565282#565282
pub fn intersect<N>(
    start: Coordinate<N>,
    end: Coordinate<N>,
    line: &LineString<N>,
    exclude_points: bool,
) -> Option<IntersectionResult<N>>
where
    N: CoordFloat,
{
    let mut intersection_u = N::from(1.0).unwrap();
    let mut intersection_t = None;
    let mut intersection_point = None;
    let mut intersection_index = None;
    let s = Coordinate {
        x: end.x - start.x,
        y: end.y - start.y,
    };

    for idx in 0..(line.0.len() - 1) {
        let p0 = line.0[idx];
        let p1 = line.0[idx + 1];
        let r = Coordinate {
            x: p1.x - p0.x,
            y: p1.y - p0.y,
        };
        let rxs = cross_product(r, s);
        if rxs.abs() < N::epsilon() {
            continue;
        }
        let q_p = Coordinate {
            x: start.x - p0.x,
            y: start.y - p0.y,
        };
        let u = cross_product(q_p, r) / rxs;
        if u < N::epsilon() || u > intersection_u {
            continue;
        }
        let t = cross_product(q_p, s) / rxs;
        if (!exclude_points && (t.is_sign_negative() || t > N::from(1.0).unwrap()))
            || (exclude_points && (t < N::from(0.00001).unwrap() || t > N::from(0.999999).unwrap()))
        {
            continue;
        }
        intersection_u = u;
        intersection_t = Some(t);
        intersection_point = Some(Coordinate {
            x: start.x + u * s.x,
            y: start.y + u * s.y,
        });
        intersection_index = Some(idx);
    }

    intersection_point.map(|point| IntersectionResult {
        u: intersection_u,
        t: intersection_t.unwrap(),
        point,
        index: intersection_index.unwrap(),
    })
}
