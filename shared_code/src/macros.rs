#[macro_export]
macro_rules! query_2 {
    ($first:ty, $second:ty, $world:ident, $query_holder: ident) => {
        let mut first_component = $world.borrow_component_vec::<$first>().unwrap();
        let mut second_component = $world.borrow_component_vec::<$second>().unwrap();
        let zip = first_component.iter_mut().zip(second_component.iter_mut());
        $query_holder = zip.filter_map(|((pcc, cc))|Some((pcc.as_mut()?, cc.as_mut()?)));
    }
}

#[macro_export]
macro_rules! query_3 {
    ($first:ty, $second:ty, $third:ty, $world:ident, $query_holder: ident) => {
        let mut first_component = $world.borrow_component_vec::<$first>().unwrap();
        let mut second_component = $world.borrow_component_vec::<$second>().unwrap();
        let mut third_component = $world.borrow_component_vec::<$third>().unwrap();
        let zip = first_component.iter_mut().zip(second_component.iter_mut()).zip(third_component.iter_mut());
        $query_holder = zip.filter_map(|((pcc, cc), tc)|Some((pcc.as_mut()?, cc.as_mut()?, tc.as_mut()?)));

    }
}

#[macro_export]
macro_rules! query_4 {
    ($first:ty, $second:ty, $third:ty, $fourth: ty, $world:ident, $query_holder: ident) => {
        let mut first_component = $world.borrow_component_vec::<$first>().unwrap();
        let mut second_component = $world.borrow_component_vec::<$second>().unwrap();
        let mut third_component = $world.borrow_component_vec::<$third>().unwrap();
        let mut fourth_component = $world.borrow_component_vec::<$fourth>().unwrap();
        let zip = first_component.iter_mut().zip(second_component.iter_mut()).zip(third_component.iter_mut()).zip(fourth_component.iter_mut());
        $query_holder = zip.filter_map(|(((pcc, cc), tc), fc)|Some((pcc.as_mut()?, cc.as_mut()?, tc.as_mut()?, fc.as_mut()?)));

    }
}
