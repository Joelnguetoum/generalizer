use crate::generaliser::generaliser::Generaliser;
use crate::matching::matching_ac::match_modulo_ac;

pub fn minimise_ac(generalisers: Vec<Generaliser>) -> Vec<Generaliser> {
    let mut r: Vec<Generaliser> = Vec::new();

    // -----------------------------
    // Step 1: Remove duplicates
    // -----------------------------
    for s in &generalisers {
        let mut found = false;

        for r_item in &r {
            // If the terms are mutually AC-matchable, we consider them duplicates
            if match_modulo_ac(&r_item.t, &s.t) && match_modulo_ac(&s.t, &r_item.t) {
                found = true;
                break;
            }
        }

        if !found {
            r.push(s.clone());
        }
    }

    // -----------------------------
    // Step 2: Remove subsumed elements
    // -----------------------------

    /*
    for el in &r{
        println!("{}", el.t);
    }
     */
    let mut s_min: Vec<Generaliser> = Vec::new();

    for s in &r {

        let mut redundant = false;

        for r_item in &r {
            // Skip self-comparison
            //println!("Before:: {} <? {} : {}",r_item.t,s.t,match_modulo_ac(&r_item.t, &s.t));

            if std::ptr::eq(s, r_item) {
                continue;
            }

            // If r_item.t AC-matches s.t, then s is redundant (subsumed)

            //println!("After:: {} <? {} : {}",r_item.t,s.t,match_modulo_ac(&r_item.t, &s.t));
            /*
            if match_modulo_ac(&r_item.t, &s.t) {
                redundant = true;
                break;
            }
             */
            if match_modulo_ac(&s.t,&r_item.t) {
                redundant = true;
                break;
            }
        }

        if !redundant {
            s_min.push(s.clone());
        }
    }

    s_min
}