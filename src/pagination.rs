use std::fmt::Debug;

use serde::Serialize;

#[derive(FromForm, Serialize, Debug, Clone)]
pub struct Pager {
    pub cur: i64,
    pub size: i64,
}

impl Pager {
    pub fn new(c: i64, s: i64) -> Self {
        Self { cur: c, size: s }
    }
    pub fn build(&self, total: i64) -> (i64, i64) {
        let limit = self.limit();

        // offset must bigger than zero, and lower than total
        let offset = (self.cur - 1) * limit;
        if offset < 0 {
            return (limit, 0);
        }
        if offset > total {
            return (limit, limit * (total / limit - 1));
        }
        return (limit, offset);
    }

    pub fn limit(&self) -> i64 {
        // page size should between 12 to 120
        if self.size < 12 {
            return 12;
        }
        if self.size > 120 {
            return 120;
        }
        return self.size;
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Pagination<T: Serialize + Debug + Clone> {
    pub previous: i64,
    pub next: i64,
    pub current: i64,
    pub size: i64,
    pub pages: Vec<i64>,
    pub items: Vec<T>,
}

impl<T: Serialize + Debug + Clone> Pagination<T> {
    fn total(total: i64, size: i64) -> i64 {
        let i = total / size + 1;
        if total % size == 0 {
            return i;
        }
        return i + 1;
    }

    fn current(cur: i64, total: i64) -> i64 {
        if cur < 1 {
            return 1;
        }
        if cur > total {
            return total;
        }
        return cur;
    }

    fn previous(cur: i64) -> i64 {
        let i = cur - 1;
        if i < 1 {
            return 1;
        }
        return i;
    }

    fn next(cur: i64, total: i64) -> i64 {
        let i = cur + 1;
        if i > total {
            return total;
        }
        return i;
    }

    fn pages(cur: i64, total: i64) -> (i64, i64) {
        let pad = 6;
        if pad * 2 >= total {
            return (1, total);
        }
        if cur - pad <= 0 {
            return (1, pad * 2);
        }
        if cur + pad <= total {
            return (cur - pad, cur + pad);
        }
        (total - pad * 2, total)
    }

    pub fn new(total: i64, pager: &Pager, items: Vec<T>) -> Self {
        // page size
        let size = pager.limit();
        let total = Self::total(total, size);
        let current = Self::current(pager.cur, total);
        let previous = Self::previous(current);
        let next = Self::next(current, total);
        let (begin, end) = Self::pages(current, total);

        // href build
        // let build = |i: i64| {
        //     format!(
        //         "{href}?size={size}&index={index}",
        //         href = href,
        //         size = s,
        //         index = i
        //     )
        // };

        Self {
            current: current,
            previous: previous,
            next: next,
            size: size,
            items: items,
            pages: (begin..end).into_iter().collect(),
        }
    }
}
