#[must_use]
pub(crate) fn class_list<'a, T>(list: T) -> String
where
    T: IntoIterator<Item = (&'a str, bool)>,
{
    list.into_iter()
        .filter(|(_, v)| *v)
        .map(|(s, _)| s)
        .collect::<Vec<&'a str>>()
        .join(" ")
}
