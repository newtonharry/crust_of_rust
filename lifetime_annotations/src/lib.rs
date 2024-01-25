trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimeter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self)
            .map(|delim_start| (delim_start, delim_start + self.len()))
    }
}

impl Delimeter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(*self)
            .map(|delim_start| (delim_start, delim_start + 1))
    }
}

#[derive(Default)]
struct Splitter<'haystack, D> {
    haystack: Option<&'haystack str>,
    delimeter: D,
}

impl<'haystack, D> Splitter<'haystack, D> {
    fn new(text: &'haystack str, pat: D) -> Self {
        Self {
            haystack: Some(text),
            delimeter: pat,
        }
    }
}

impl<'haystack, D> Iterator for Splitter<'haystack, D>
where
    D: Delimeter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.haystack.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimeter.find_next(remainder) {
            let text = Some(&remainder[..delim_start]);
            *remainder = &remainder[delim_end..];
            text
        } else {
            self.haystack.take()
        }
    }
}

fn until_char<D>(s: &str, c: D) -> &str
where
    D: Delimeter,
{
    Splitter::new(s, c).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitter_test() {
        let mut splitter = Splitter::new("hi there", "e");

        assert_eq!(splitter.next(), Some("hi th"));
        assert_eq!(splitter.next(), Some("r"));
        assert_eq!(splitter.next(), Some(""));
        assert_eq!(splitter.next(), None);
    }

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello world", 'o'), "hell")
    }
}
