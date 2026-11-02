#[cfg(test)]
mod tests {
    use geo_types::geometry::Point;

    #[test]
    fn making_a_point() {
        let pt = Point::new(45.5, -122.8);
        println!("{:?}", pt);
    }
}
