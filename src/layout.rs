//! Independent 2-D educational PV panel packing engine (millimetres).
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Orientation { Auto, Portrait, Landscape }
#[derive(Clone,Copy,Debug)]
pub struct Parameters {
    pub roof_width:f64, pub roof_height:f64, pub module_width:f64,
    pub module_height:f64, pub module_watt:f64, pub gap:f64,
    pub setback:f64, pub origin_x:f64, pub origin_y:f64,
    pub orientation:Orientation,
}
impl Default for Parameters {
    fn default()->Self {
        Self {roof_width:10000.,roof_height:6500.,module_width:1134.,
          module_height:1722.,module_watt:450.,gap:20.,setback:350.,
          origin_x:0.,origin_y:0.,orientation:Orientation::Auto}
    }
}
#[derive(Clone,Copy,Debug)]
pub struct Panel {pub x:f64,pub y:f64,pub width:f64,pub height:f64,pub row:usize,pub column:usize}
#[derive(Debug)]
pub struct Layout {
    pub panels:Vec<Panel>,pub columns:usize,pub rows:usize,
    pub portrait:bool,pub total_watt:f64,pub coverage_percent:f64
}
fn along(span:f64,module:f64,gap:f64)->usize {
    if span<module {0} else {((span+gap)/(module+gap)).floor().max(0.) as usize}
}
pub fn compute(p:&Parameters)->Result<Layout,&'static str> {
    let v=[p.roof_width,p.roof_height,p.module_width,p.module_height,p.module_watt,
           p.gap,p.setback,p.origin_x,p.origin_y];
    if v.iter().any(|v| !v.is_finite()) {return Err("Hodnoty musejí být konečná čísla.");}
    if p.roof_width<=0. || p.roof_height<=0. || p.module_width<=0.
       || p.module_height<=0. || p.module_watt<=0. || p.gap<0. || p.setback<0. {
        return Err("Rozměry a výkon musejí být kladné, mezera/odstup nezáporné.");
    }
    if p.roof_width>1_000_000. || p.roof_height>1_000_000. ||
       p.module_width>20_000. || p.module_height>20_000. {
        return Err("Rozměry jsou mimo výukový rozsah.");
    }
    let uw=p.roof_width-2.*p.setback;
    let uh=p.roof_height-2.*p.setback;
    let pc=along(uw,p.module_width,p.gap);
    let pr=along(uh,p.module_height,p.gap);
    let lc=along(uw,p.module_height,p.gap);
    let lr=along(uh,p.module_width,p.gap);
    let portrait=match p.orientation {
        Orientation::Portrait=>true,Orientation::Landscape=>false,
        Orientation::Auto=>pc.saturating_mul(pr)>=lc.saturating_mul(lr),
    };
    let (w,h,nc,nr)=if portrait {
        (p.module_width,p.module_height,pc,pr)
    } else {(p.module_height,p.module_width,lc,lr)};
    let count=nc.saturating_mul(nr);
    if count==0 {return Err("Na střechu se nevejde žádný panel.");}
    if count>500 {return Err("Výukový limit je 500 panelů.");}
    let occupied_w=nc as f64*w+(nc-1) as f64*p.gap;
    let occupied_h=nr as f64*h+(nr-1) as f64*p.gap;
    let sx=p.origin_x+(p.roof_width-occupied_w)/2.;
    let sy=p.origin_y+(p.roof_height-occupied_h)/2.;
    let mut panels=Vec::with_capacity(count);
    for row in 0..nr {for col in 0..nc {
        panels.push(Panel {
            x:sx+col as f64*(w+p.gap),y:sy+row as f64*(h+p.gap),
            width:w,height:h,row:row+1,column:col+1
        });
    }}
    Ok(Layout{panels,columns:nc,rows:nr,portrait,total_watt:count as f64*p.module_watt,
       coverage_percent:count as f64*w*h/(p.roof_width*p.roof_height)*100.})
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn default_layout() {
        let p=Parameters::default();
        let l=compute(&p).unwrap();
        assert_eq!((l.columns,l.rows,l.panels.len()),(5,5,25));
        assert!(!l.portrait);
        assert!((l.total_watt-11250.).abs()<0.001);
        for m in &l.panels {
            assert!(m.x>=p.setback-0.0001);
            assert!(m.y>=p.setback-0.0001);
            assert!(m.x+m.width<=p.roof_width-p.setback+0.0001);
            assert!(m.y+m.height<=p.roof_height-p.setback+0.0001);
        }
    }
    #[test] fn portrait_layout(){
        let p=Parameters{orientation:Orientation::Portrait,..Parameters::default()};
        let l=compute(&p).unwrap();
        assert_eq!((l.columns,l.rows,l.panels.len()),(8,3,24));
    }
    #[test] fn errors(){
        assert!(compute(&Parameters{gap:f64::NAN,..Parameters::default()}).is_err());
        assert!(compute(&Parameters{roof_width:100.,..Parameters::default()}).is_err());
        assert!(compute(&Parameters{module_watt:0.,..Parameters::default()}).is_err());
    }
}
