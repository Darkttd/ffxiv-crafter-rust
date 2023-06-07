use core::panic;
use std::collections::HashMap;
use std::iter::FromIterator;
use crate::state::{CraftParameter, CraftState, StatusCondition, all_status_conditions};
use lazy_static::lazy_static;

// https://docs.google.com/document/d/1Fl5X16oPF-4X29v5PukCkVgcPhZsbzhKcoq6Us7fhKU/edit#
// https://docs.google.com/spreadsheets/d/125HaSQnewXoDkjiOArtHNWEttwN-GEmipHyIW5Dbw90/edit#gid=247478737

lazy_static! {
    static ref LEVEL_TABLE: HashMap<i64, i64> = HashMap::<i64, i64>::from_iter(IntoIterator::into_iter([
        (51, 120),
        (52, 125),
        (53, 130),
        (54, 133),
        (55, 136),
        (56, 139),
        (57, 142),
        (58, 145),
        (59, 148),
        (60, 150),
        (61, 260),
        (62, 265),
        (63, 270),
        (64, 273),
        (65, 276),
        (66, 279),
        (67, 282),
        (68, 285),
        (69, 288),
        (70, 290),
        (71, 390),
        (72, 395),
        (73, 400),
        (74, 403),
        (75, 406),
        (76, 409),
        (77, 412),
        (78, 415),
        (79, 418),
        (80, 420),
        (81, 517),
        (82, 520),
        (83, 525),
        (84, 530),
        (85, 535),
        (86, 540),
        (87, 545),
        (88, 550),
        (89, 555),
        (90, 560),
    ]));
}

lazy_static! {
    static ref PROGRESS_DIV_TABLE: HashMap<i64, i64> = HashMap::<i64, i64>::from_iter(IntoIterator::into_iter([
        (1, 50),
        (2, 50),
        (3, 50),
        (4, 50),
        (5, 50),
        (6, 50),
        (7, 50),
        (8, 50),
        (9, 50),
        (10, 50),
        (11, 50),
        (12, 50),
        (13, 50),
        (14, 50),
        (15, 50),
        (16, 50),
        (17, 50),
        (18, 50),
        (19, 50),
        (20, 50),
        (21, 50),
        (22, 50),
        (23, 50),
        (24, 50),
        (25, 50),
        (26, 50),
        (27, 50),
        (28, 50),
        (29, 50),
        (30, 50),
        (31, 50),
        (32, 50),
        (33, 50),
        (34, 50),
        (35, 50),
        (36, 50),
        (37, 50),
        (38, 50),
        (39, 50),
        (40, 50),
        (41, 50),
        (42, 50),
        (43, 50),
        (44, 50),
        (45, 50),
        (46, 50),
        (47, 50),
        (48, 50),
        (49, 50),
        (50, 50),
        (55, 50),
        (70, 50),
        (90, 50),
        (110, 50),
        (115, 61),
        (125, 62),
        (130, 63),
        (133, 64),
        (136, 65),
        (139, 66),
        (142, 67),
        (145, 68),
        (148, 69),
        (150, 70),
        (160, 70),
        (180, 70),
        (210, 70),
        (220, 70),
        (250, 70),
        (255, 81),
        (265, 82),
        (270, 83),
        (273, 84),
        (276, 85),
        (279, 86),
        (282, 87),
        (285, 88),
        (288, 89),
        (290, 90),
        (300, 90),
        (320, 90),
        (350, 90),
        (380, 90),
        (390, 101),
        (395, 102),
        (400, 103),
        (403, 104),
        (406, 105),
        (409, 106),
        (412, 107),
        (415, 108),
        (418, 109),
        (430, 110),
        (440, 110),
        (450, 110),
        (480, 110),
        (481, 140),
        (490, 110),
        (511, 140),
        (512, 140),
        (513, 140),
        (514, 140),
        (515, 140),
        (516, 140),
        (517, 121),
        (520, 122),
        (525, 123),
        (530, 124),
        (535, 125),
        (540, 126),
        (545, 127),
        (550, 128),
        (555, 129),
        (560, 130),
        (570, 130),
        (580, 130),
        (590, 130),
        (610, 130),
        (611, 180),
        (620, 130),
        (640, 130),
        (641, 180),
    ]));
}  

lazy_static! {
    static ref PROGRESS_MOD_TABLE: HashMap<i64, i64> = HashMap::<i64, i64>::from_iter(IntoIterator::into_iter([
        (1, 100),
        (2, 100),
        (3, 100),
        (4, 100),
        (5, 100),
        (6, 100),
        (7, 100),
        (8, 100),
        (9, 100),
        (10, 100),
        (11, 100),
        (12, 100),
        (13, 100),
        (14, 100),
        (15, 100),
        (16, 100),
        (17, 100),
        (18, 100),
        (19, 100),
        (20, 100),
        (21, 100),
        (22, 100),
        (23, 100),
        (24, 100),
        (25, 100),
        (26, 100),
        (27, 100),
        (28, 100),
        (29, 100),
        (30, 100),
        (31, 100),
        (32, 100),
        (33, 100),
        (34, 100),
        (35, 100),
        (36, 100),
        (37, 100),
        (38, 100),
        (39, 100),
        (40, 100),
        (41, 100),
        (42, 100),
        (43, 100),
        (44, 100),
        (45, 100),
        (46, 100),
        (47, 100),
        (48, 100),
        (49, 100),
        (50, 100),
        (55, 100),
        (70, 80),
        (90, 80),
        (110, 80),
        (115, 100),
        (125, 100),
        (130, 100),
        (133, 100),
        (136, 100),
        (139, 100),
        (142, 100),
        (145, 100),
        (148, 100),
        (150, 100),
        (160, 100),
        (180, 80),
        (210, 80),
        (220, 80),
        (250, 80),
        (255, 100),
        (265, 100),
        (270, 100),
        (273, 100),
        (276, 100),
        (279, 100),
        (282, 100),
        (285, 100),
        (288, 100),
        (290, 100),
        (300, 100),
        (320, 80),
        (350, 80),
        (380, 80),
        (390, 100),
        (395, 100),
        (400, 100),
        (403, 100),
        (406, 100),
        (409, 100),
        (412, 100),
        (415, 100),
        (418, 100),
        (430, 100),
        (440, 100),
        (450, 80),
        (480, 80),
        (481, 100),
        (490, 80),
        (511, 100),
        (512, 100),
        (513, 100),
        (514, 100),
        (515, 100),
        (516, 100),
        (517, 100),
        (520, 100),
        (525, 100),
        (530, 100),
        (535, 100),
        (540, 100),
        (545, 100),
        (550, 100),
        (555, 100),
        (560, 90),
        (570, 90),
        (580, 80),
        (590, 80),
        (610, 80),
        (611, 100),
        (620, 80),
        (640, 80),
        (641, 100),
    ]));
}  

lazy_static! {
    static ref QUALITY_DIV_TABLE: HashMap<i64, i64> = HashMap::<i64, i64>::from_iter(IntoIterator::into_iter([
        (1, 30),
        (2, 30),
        (3, 30),
        (4, 30),
        (5, 30),
        (6, 30),
        (7, 30),
        (8, 30),
        (9, 30),
        (10, 30),
        (11, 30),
        (12, 30),
        (13, 30),
        (14, 30),
        (15, 30),
        (16, 30),
        (17, 30),
        (18, 30),
        (19, 30),
        (20, 30),
        (21, 30),
        (22, 30),
        (23, 30),
        (24, 30),
        (25, 30),
        (26, 30),
        (27, 30),
        (28, 30),
        (29, 30),
        (30, 30),
        (31, 30),
        (32, 30),
        (33, 30),
        (34, 30),
        (35, 30),
        (36, 30),
        (37, 30),
        (38, 30),
        (39, 30),
        (40, 30),
        (41, 30),
        (42, 30),
        (43, 30),
        (44, 30),
        (45, 30),
        (46, 30),
        (47, 30),
        (48, 30),
        (49, 30),
        (50, 30),
        (55, 30),
        (70, 30),
        (90, 30),
        (110, 30),
        (115, 39),
        (125, 43),
        (130, 43),
        (133, 44),
        (136, 45),
        (139, 46),
        (142, 47),
        (145, 48),
        (148, 49),
        (150, 50),
        (160, 50),
        (180, 50),
        (210, 50),
        (220, 50),
        (250, 50),
        (255, 58),
        (265, 61),
        (270, 64),
        (273, 65),
        (276, 66),
        (279, 66),
        (282, 68),
        (285, 68),
        (288, 68),
        (290, 70),
        (300, 70),
        (320, 70),
        (350, 70),
        (380, 70),
        (390, 81),
        (395, 82),
        (400, 83),
        (403, 84),
        (406, 85),
        (409, 86),
        (412, 87),
        (415, 88),
        (418, 89),
        (430, 90),
        (440, 90),
        (450, 90),
        (480, 90),
        (481, 130),
        (490, 90),
        (511, 130),
        (512, 130),
        (513, 130),
        (514, 130),
        (515, 130),
        (516, 130),
        (517, 105),
        (520, 106),
        (525, 107),
        (530, 108),
        (535, 109),
        (540, 110),
        (545, 111),
        (550, 112),
        (555, 113),
        (560, 115),
        (570, 115),
        (580, 115),
        (590, 115),
        (610, 115),
        (611, 180),
        (620, 115),
        (640, 115),
        (641, 180),
    ]));
}  

lazy_static! {
    static ref QUALITY_MOD_TABLE: HashMap<i64, i64> = HashMap::<i64, i64>::from_iter(IntoIterator::into_iter([
        (1, 100),
        (2, 100),
        (3, 100),
        (4, 100),
        (5, 100),
        (6, 100),
        (7, 100),
        (8, 100),
        (9, 100),
        (10, 100),
        (11, 100),
        (12, 100),
        (13, 100),
        (14, 100),
        (15, 100),
        (16, 100),
        (17, 100),
        (18, 100),
        (19, 100),
        (20, 100),
        (21, 100),
        (22, 100),
        (23, 100),
        (24, 100),
        (25, 100),
        (26, 100),
        (27, 100),
        (28, 100),
        (29, 100),
        (30, 100),
        (31, 100),
        (32, 100),
        (33, 100),
        (34, 100),
        (35, 100),
        (36, 100),
        (37, 100),
        (38, 100),
        (39, 100),
        (40, 100),
        (41, 100),
        (42, 100),
        (43, 100),
        (44, 100),
        (45, 100),
        (46, 100),
        (47, 100),
        (48, 100),
        (49, 100),
        (50, 100),
        (55, 100),
        (70, 70),
        (90, 70),
        (110, 70),
        (115, 100),
        (125, 100),
        (130, 100),
        (133, 100),
        (136, 100),
        (139, 100),
        (142, 100),
        (145, 100),
        (148, 100),
        (150, 100),
        (160, 100),
        (180, 70),
        (210, 70),
        (220, 70),
        (250, 70),
        (255, 100),
        (265, 100),
        (270, 100),
        (273, 100),
        (276, 100),
        (279, 100),
        (282, 100),
        (285, 100),
        (288, 100),
        (290, 100),
        (300, 100),
        (320, 70),
        (350, 70),
        (380, 70),
        (390, 100),
        (395, 100),
        (400, 100),
        (403, 100),
        (406, 100),
        (409, 100),
        (412, 100),
        (415, 100),
        (418, 100),
        (430, 100),
        (440, 100),
        (450, 70),
        (480, 70),
        (481, 100),
        (490, 70),
        (511, 100),
        (512, 100),
        (513, 100),
        (514, 100),
        (515, 100),
        (516, 100),
        (517, 100),
        (520, 100),
        (525, 100),
        (530, 100),
        (535, 100),
        (540, 100),
        (545, 100),
        (550, 100),
        (555, 100),
        (560, 80),
        (570, 80),
        (580, 70),
        (590, 70),
        (610, 70),
        (611, 100),
        (620, 70),
        (640, 70),
        (641, 100),
    ]));
}  

pub fn crafting_level(job_level: i64) -> i64 {
    if job_level <= 50 {
        job_level
    } else {
        LEVEL_TABLE[&job_level]
    }
}

pub fn progress_div(recipe_level: i64) -> i64 {
    PROGRESS_DIV_TABLE[&recipe_level]
}

pub fn progress_mod(recipe_level: i64) -> i64 {
    PROGRESS_MOD_TABLE[&recipe_level]
}

pub fn quality_div(recipe_level: i64) -> i64 {
    QUALITY_DIV_TABLE[&recipe_level]
}

pub fn quality_mod(recipe_level: i64) -> i64 {
    QUALITY_MOD_TABLE[&recipe_level]
}

lazy_static! {
    static ref CONDITION_MASK_TABLE: HashMap<i64, i64> = HashMap::<i64, i64>::from_iter(IntoIterator::into_iter([
        (1, 15),
        (2, 15),
        (3, 15),
        (4, 15),
        (5, 15),
        (6, 15),
        (7, 15),
        (8, 15),
        (9, 15),
        (10, 15),
        (11, 15),
        (12, 15),
        (13, 15),
        (14, 15),
        (15, 15),
        (16, 15),
        (17, 15),
        (18, 15),
        (19, 15),
        (20, 15),
        (21, 15),
        (22, 15),
        (23, 15),
        (24, 15),
        (25, 15),
        (26, 15),
        (27, 15),
        (28, 15),
        (29, 15),
        (30, 15),
        (31, 15),
        (32, 15),
        (33, 15),
        (34, 15),
        (35, 15),
        (36, 15),
        (37, 15),
        (38, 15),
        (39, 15),
        (40, 15),
        (41, 15),
        (42, 15),
        (43, 15),
        (44, 15),
        (45, 15),
        (46, 15),
        (47, 15),
        (48, 15),
        (49, 15),
        (50, 15),
        (55, 15),
        (70, 15),
        (90, 15),
        (110, 15),
        (115, 15),
        (125, 15),
        (130, 15),
        (133, 15),
        (136, 15),
        (139, 15),
        (142, 15),
        (145, 15),
        (148, 15),
        (150, 15),
        (160, 15),
        (180, 15),
        (210, 15),
        (220, 15),
        (250, 15),
        (255, 15),
        (265, 15),
        (270, 15),
        (273, 15),
        (276, 15),
        (279, 15),
        (282, 15),
        (285, 15),
        (288, 15),
        (290, 15),
        (300, 15),
        (320, 15),
        (350, 15),
        (380, 15),
        (390, 15),
        (395, 15),
        (400, 15),
        (403, 15),
        (406, 15),
        (409, 15),
        (412, 15),
        (415, 15),
        (418, 15),
        (430, 15),
        (440, 15),
        (450, 15),
        (480, 15),
        (481, 115),
        (490, 15),
        (511, 115),
        (512, 115),
        (513, 483),
        (514, 115),
        (515, 483),
        (516, 499),
        (517, 15),
        (520, 15),
        (525, 15),
        (530, 15),
        (535, 15),
        (540, 15),
        (545, 15),
        (550, 15),
        (555, 15),
        (560, 15),
        (570, 15),
        (580, 15),
        (590, 15),
        (610, 15),
        (611, 435),
        (620, 15),
        (640, 15),
        (641, 995),
    ]));
}  

fn condition_mask(recipe_level: i64) -> i64 {
    CONDITION_MASK_TABLE[&recipe_level]
}

fn is_expert_recipe(recipe_level: i64) -> bool {
    condition_mask(recipe_level) != 15
}

fn expert_condition_probability(condition: StatusCondition) -> f64 {
    match condition {
        StatusCondition::NORMAL => panic!("don't obtain normal probability from a table. it must be calculated as a remainder of other probas."),
        StatusCondition::GOOD => 0.12,
        StatusCondition::EXCELLENT => 0.,
        StatusCondition::POOR => 0.,
        StatusCondition::CENTRED => 0.15,
        StatusCondition::PLIANT => 0.12,
        StatusCondition::STURDY => 0.15,
        StatusCondition::MALLEABLE => 0.12,
        StatusCondition::PRIMED => 0.12,
        StatusCondition::GOOD_OMEN => 0.12,  // todo: check this value
    }
}

fn condition_bit(condition: StatusCondition) -> i64 {
    match condition {
        StatusCondition::NORMAL => 1,
        StatusCondition::GOOD => 2,
        StatusCondition::EXCELLENT => 4,
        StatusCondition::POOR => 8,
        StatusCondition::CENTRED => 16,
        StatusCondition::STURDY => 32,
        StatusCondition::PLIANT => 64,
        StatusCondition::MALLEABLE => 128,
        StatusCondition::PRIMED => 256,
        StatusCondition::GOOD_OMEN => 512,  // todo: check this value
    }
}

pub fn transition_probabilities(params: &CraftParameter, state: &CraftState) -> HashMap<StatusCondition, f64> {
    if is_expert_recipe(params.item.recipe_level) {
        expert_recipe_transition_probabilities(params, state)
    } else {
        normal_recipe_transition_probabilities(params, state)
    }
}

fn expert_recipe_transition_probabilities(params: &CraftParameter, state: &CraftState) -> HashMap<StatusCondition, f64> {
    if state.condition == StatusCondition::GOOD_OMEN {
        return HashMap::<StatusCondition, f64>::from_iter(IntoIterator::into_iter([
            (StatusCondition::GOOD, 1.),
        ]));
    }

    let recipe_level = params.item.recipe_level;
    let mask = condition_mask(recipe_level);
    let mut normal_proba = 1.;

    let mut probas = HashMap::new();
    for condition in all_status_conditions() {
        if condition == StatusCondition::NORMAL {
            continue;
        }
        let bit = condition_bit(condition);
        if mask & bit != 0 {
            let proba = expert_condition_probability(condition);
            normal_proba -= proba;
            probas.insert(condition, proba);
        }
    }

    probas.insert(StatusCondition::NORMAL, normal_proba);
    probas
}

fn normal_recipe_transition_probabilities(params: &CraftParameter, state: &CraftState) -> HashMap<StatusCondition, f64> {
    if state.condition == StatusCondition::EXCELLENT {
        return HashMap::<StatusCondition, f64>::from_iter(IntoIterator::into_iter([
            (StatusCondition::POOR, 1.),
        ]));
    }
    if state.condition == StatusCondition::GOOD || state.condition == StatusCondition::POOR {
        return HashMap::<StatusCondition, f64>::from_iter(IntoIterator::into_iter([
            (StatusCondition::NORMAL, 1.),
        ]));
    }
    if state.condition == StatusCondition::NORMAL {
        let good_proba = if params.player.job_level >= 63 { 0.25 } else { 0.2 };
        let excellent_proba = 0.04;
        let normal_proba = 1. - good_proba - excellent_proba;
        return HashMap::<StatusCondition, f64>::from_iter(IntoIterator::into_iter([
            (StatusCondition::NORMAL, normal_proba),
            (StatusCondition::GOOD, good_proba),
            (StatusCondition::EXCELLENT, excellent_proba),
        ]));
    }
    panic!("invalid StatusCondition");
}
