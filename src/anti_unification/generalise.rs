use crate::anti_unification::configuration::generalisation_process::GeneralisationProcess;

use crate::anti_unification::generaliser::generaliser::Generaliser;

/*  */
impl GeneralisationProcess {
    pub fn generalise(&mut self,alpuente: bool,verbose: bool) -> Vec<Generaliser> {


        /**/
        while let Some(config) = self.unsolved_configurations.pop_back() {

            self.process_configuration(config,false,alpuente,verbose,false);

        }


        /*
        while !self.unsolved_configurations.is_empty(){

            self.unsolved_configurations.par_iter_mut().map(|x| self.process_configuration(x.clone(), false,alpuente,verbose));
        }
         */

        self.to_generalisers()
    }

}




//Work in progress
/*
impl GeneralisationEngine{
    pub fn generalise(&mut self,alpuente: bool,verbose: bool) -> Vec<Generaliser> {

        self.run(false,alpuente,verbose);

        self.to_generalisers()
    }
}
 */
