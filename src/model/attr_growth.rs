#[derive(Clone)]
pub struct AttrGrowth {
    pub growth_dice: [[i32; 6]; 6],
    pub growth: [[i32; 6]; 6],
    pub attrs: [i32; 6],
    pub attr_mods: [i32; 6],
}

impl AttrGrowth {
    pub fn validate_attr(attr: usize) -> bool {
        attr < 6
    }

    pub fn count_used(&self, attr: [usize; 2]) -> i32 {
        let [p, s] = attr;
        if p == s {
            self.growth[p][s]
        } else {
            self.growth[p][s] + self.growth[s][p]
        }
    }

    pub fn count_used_with_growth(growth: &[[i32; 6]; 6], attr: [usize; 2]) -> i32 {
        let [p, s] = attr;
        if p == s {
            growth[p][s]
        } else {
            growth[p][s] + growth[s][p]
        }
    }

    pub fn count_growthable(&self, count_used: bool, attr: [usize; 2]) -> i32 {
        self.count_growthable_with_cxclude(
            count_used,
            attr,
            [false, false, false, false, false, false],
        )
    }

    pub fn count_growthable_with_cxclude(
        &self,
        count_used: bool,
        attr: [usize; 2],
        mut exclude: [bool; 6],
    ) -> i32 {
        let [p, s] = attr;
        if !Self::validate_attr(p) || !Self::validate_attr(s) {
            return 0;
        }

        exclude[p] = true;
        exclude[s] = true;

        let mut count = self.growth_dice[usize::min(p, s)][usize::max(p, s)]
            - if count_used {
                self.count_used([p, s])
            } else {
                0
            };

        if p != s {
            for i in 0..6 {
                if !exclude[i] {
                    count += i32::min(
                        self.growth_dice[usize::min(p, i)][usize::max(p, i)]
                            - self.count_used([p, i]),
                        self.count_growthable_with_cxclude(count_used, [i, s], exclude.clone()),
                    );
                }
            }
        }

        count
    }

    pub fn clone_growth(growth: &mut [[i32; 6]; 6], other: &[[i32; 6]; 6]) {
        for i in 0..6 {
            for j in 0..6 {
                growth[i][j] = other[i][j];
            }
        }
    }

    pub fn growth(&mut self, attr: [usize; 2]) -> bool {
        let mut growth = self.growth.clone();
        if self.growth_with_exclude(
            attr,
            [false, false, false, false, false, false],
            &mut growth,
        ) {
            Self::clone_growth(&mut self.growth, &growth);
            true
        } else {
            false
        }
    }

    fn growth_with_exclude(
        &self,
        attr: [usize; 2],
        mut exclude: [bool; 6],
        growth: &mut [[i32; 6]; 6],
    ) -> bool {
        let [p, s] = attr;
        if !Self::validate_attr(p) || !Self::validate_attr(s) {
            return false;
        }

        exclude[p] = true;
        exclude[s] = true;

        if self.growth_dice[usize::min(p, s)][usize::max(p, s)]
            > Self::count_used_with_growth(&growth, [p, s])
        {
            growth[p][s] += 1;
            return true;
        }

        if growth[s][p] > 0 {
            growth[p][s] += 1;
            growth[s][p] -= 1;
            return true;
        }

        if p != s {
            for i in 0..6 {
                if !exclude[i] {
                    if self.count_growthable_with_cxclude(false, [p, i], exclude.clone()) > 0
                        && self.count_growthable_with_cxclude(false, [i, s], exclude.clone()) > 0
                    {
                        let mut growth_tmp = growth.clone();
                        if self.growth_with_exclude([i, s], exclude.clone(), &mut growth_tmp) {
                            if self.growth_with_exclude([p, i], exclude.clone(), &mut growth_tmp) {
                                Self::clone_growth(growth, &growth_tmp);
                                return true;
                            }
                        }
                    }
                }
            }
        }

        return false;
    }
}
