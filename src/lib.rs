//! Solar CAD CZ - Vyuka v0.1.0 (Open CAD Studio API v14).
//! Educational use only; this does NOT certify photovoltaic installation designs.
mod layout;
use layout::{compute,Orientation,Parameters};
use std::collections::HashMap;
use std::sync::OnceLock;
use ocs_plugin_api::dialog::{DialogControl,DialogDef,DialogResponse,DialogValue};
use ocs_plugin_api::host::{acadrust,BuiltinPlugin,HostApi,LayerConfig,ExtendedDataRecord,XDataValue};
use ocs_plugin_api::manifest::{ApiVersion,PluginManifest};
use ocs_plugin_api::ribbon::{CadModule,IconKind,ModuleEvent,RibbonGroup,RibbonItem,ToolDef};
use acadrust::entities::{LwPolyline,Text};
use acadrust::types::{Color,Vector2,Vector3};
use acadrust::EntityType;

const APP:&str="SOLARCAD_CZ";
static MANIFEST:PluginManifest=PluginManifest{
 id:"cz.solarcad.education",
 name:"Solar CAD CZ - Vyuka",
 version:env!("CARGO_PKG_VERSION"),
 description:"Educational 2D PV roof panel layout",
 api_version:ApiVersion::CURRENT,
 ribbon_order:65,
 xdata_apps:&[APP],
 command_prefixes:&["SOL_"],
};
struct SolarModule;
impl CadModule for SolarModule {
 fn id(&self)->&'static str{"solar_cad_cz"}
 fn title(&self)->&'static str{"Solar CAD CZ"}
 fn ribbon_groups(&self)->&[RibbonGroup]{
  static GROUPS:OnceLock<Vec<RibbonGroup>>=OnceLock::new();
  GROUPS.get_or_init(||vec![RibbonGroup{
   title:"Fotovoltaika - vyuka",
   tools:vec![
    RibbonItem::LargeTool(ToolDef{
     id:"SOL_LAYOUT",label:"Navrhnout panely",icon:IconKind::Glyph("▦"),
     event:ModuleEvent::Command("SOL_LAYOUT".to_owned())
    }),
    RibbonItem::Tool(ToolDef{
     id:"SOL_DEMO",label:"Ukazkovy vykres",icon:IconKind::Glyph("▤"),
     event:ModuleEvent::Command("SOL_DEMO".to_owned())
    }),
    RibbonItem::Tool(ToolDef{
     id:"SOL_HELP",label:"Napoveda",icon:IconKind::Glyph("?"),
     event:ModuleEvent::Command("SOL_HELP".to_owned())
    })
   ]
  }])
 }
}
struct SolarPlugin;
impl BuiltinPlugin for SolarPlugin {
 fn manifest(&self)->&'static PluginManifest{&MANIFEST}
 fn ribbon(&self)->Box<dyn CadModule>{Box::new(SolarModule)}
 fn dispatch(&self,host:&mut dyn HostApi,cmd:&str)->bool {
  match cmd.trim().to_ascii_uppercase().as_str(){
   "SOL_LAYOUT"=>{design_dialog(host);true},
   "SOL_DEMO"=>{draw_project(host,Parameters::default());true},
   "SOL_HELP"=>{help(host);true},
   _=>false
  }
 }
}
ocs_plugin_api::export_plugin!(SolarPlugin);

fn number(id:&str,label:&str,default:f64,min:f64,max:f64)->DialogControl {
 DialogControl::Number {
  id:id.to_owned(),label:label.to_owned(),default,
  min:Some(min),max:Some(max),step:Some(1.)
 }
}
fn val(v:&HashMap<String,DialogValue>,id:&str)->Result<f64,String>{
 match v.get(id) {
  Some(DialogValue::Number(x)) if x.is_finite()=>Ok(*x),
  _=>Err(format!("Chybí číselná hodnota {id}."))
 }
}
fn design_dialog(host:&mut dyn HostApi){
 let p=Parameters::default();
 let form=DialogDef::new("Solar CAD CZ - navrh FV panelu")
  .width(490.).submit_label("Vytvorit vykres").cancel_label("Zrusit")
  .with_control(DialogControl::InfoText{
   text:"Výuková geometrie obdélníkové střechy, rozměry v mm. Není realizačním projektem.".into(),
   align_h:None
  })
  .with_control(number("rw","Sirka strechy X [mm]",p.roof_width,100.,1_000_000.))
  .with_control(number("rh","Vyska strechy Y [mm]",p.roof_height,100.,1_000_000.))
  .with_control(number("mw","Sirka panelu [mm]",p.module_width,100.,20_000.))
  .with_control(number("mh","Vyska panelu [mm]",p.module_height,100.,20_000.))
  .with_control(number("wp","Vykon panelu [Wp]",p.module_watt,1.,2000.))
  .with_control(number("gap","Mezera mezi panely [mm]",p.gap,0.,10_000.))
  .with_control(number("offset","Modelovy odstup od okraje [mm]",p.setback,0.,100_000.))
  .with_control(number("ox","Pocatek X [mm]",p.origin_x,-1_000_000.,1_000_000.))
  .with_control(number("oy","Pocatek Y [mm]",p.origin_y,-1_000_000.,1_000_000.))
  .with_control(DialogControl::Choice{
   id:"orientation".into(),label:"Orientace panelu".into(),
   options:vec!["Automaticky".into(),"Na vysku".into(),"Na sirku".into()],
   default_index:0
  });
 let v=match host.show_dialog(form){
  DialogResponse::Submitted(v)=>v,
  DialogResponse::Cancelled=>{host.push_info("Solar CAD CZ: zruseno.");return;}
 };
 let make=||->Result<Parameters,String>{
  let orientation=match v.get("orientation"){
   Some(DialogValue::Choice{index:1,..})=>Orientation::Portrait,
   Some(DialogValue::Choice{index:2,..})=>Orientation::Landscape,
   _=>Orientation::Auto
  };
  Ok(Parameters{
   roof_width:val(&v,"rw")?,roof_height:val(&v,"rh")?,
   module_width:val(&v,"mw")?,module_height:val(&v,"mh")?,
   module_watt:val(&v,"wp")?,gap:val(&v,"gap")?,
   setback:val(&v,"offset")?,origin_x:val(&v,"ox")?,
   origin_y:val(&v,"oy")?,orientation
  })
 };
 match make(){
  Ok(p)=>draw_project(host,p),
  Err(e)=>host.push_error(&format!("Solar CAD CZ: {e}"))
 }
}
fn rect(x:f64,y:f64,w:f64,h:f64,layer:&str)->EntityType{
 let mut entity=LwPolyline::from_points(vec![
  Vector2::new(x,y),Vector2::new(x+w,y),
  Vector2::new(x+w,y+h),Vector2::new(x,y+h)
 ]);
 entity.close();
 entity.common.layer=layer.to_owned();
 EntityType::LwPolyline(entity)
}
fn text_at(value:impl Into<String>,x:f64,y:f64,h:f64,layer:&str)->EntityType{
 let mut entity=Text::with_value(value,Vector3::new(x,y,0.)).with_height(h);
 entity.common.layer=layer.to_owned();
 EntityType::Text(entity)
}
fn record(values:Vec<String>)->ExtendedDataRecord {
 ExtendedDataRecord{
  application_name:APP.to_owned(),
  values:values.into_iter().map(XDataValue::String).collect()
 }
}
fn ensure_layer(host:&mut dyn HostApi,name:&str,color:Color,description:&str){
 let _=host.add_layer(LayerConfig{
  name:name.to_owned(),color:Some(color),description:Some(description.to_owned()),
  ..Default::default()
 });
}
fn draw_project(host:&mut dyn HostApi,p:Parameters){
 let result=match compute(&p){
  Ok(l)=>l,
  Err(e)=>{host.push_error(&format!("Solar CAD CZ: {e}"));return;}
 };
 host.begin_undo_group("Solar CAD CZ - vykres panelu");
 ensure_layer(host,"SOL_STRECHA",Color::Index(7),"Obrys strechy");
 ensure_layer(host,"SOL_PANELY",Color::Index(4),"FV moduly");
 ensure_layer(host,"SOL_POPISY",Color::Index(2),"Cisla a legenda");
 let mut entities=Vec::with_capacity(2*result.panels.len()+16);
 entities.push(rect(p.origin_x,p.origin_y,p.roof_width,p.roof_height,"SOL_STRECHA"));
 let mut panel_indices=Vec::with_capacity(result.panels.len());
 for m in &result.panels {
  panel_indices.push(entities.len());
  entities.push(rect(m.x,m.y,m.width,m.height,"SOL_PANELY"));
  let n=(m.row-1)*result.columns+m.column;
  entities.push(text_at(format!("P{n:03}"),m.x+0.09*m.width,
    m.y+0.1*m.height,120.,"SOL_POPISY"));
 }
 let legend=[
  "SOLAR CAD CZ - VYUKA".to_owned(),
  format!("STRECHA: {:.0} x {:.0} mm",p.roof_width,p.roof_height),
  format!("PANEL: {:.0} x {:.0} mm / {:.0} Wp",p.module_width,p.module_height,p.module_watt),
  format!("ROZLOZENI: {} x {}",result.columns,result.rows),
  format!("POCET: {} ks",result.panels.len()),
  format!("VYKON: {:.3} kWp",result.total_watt/1000.),
  format!("POKRYTI STRECHY: {:.1} %",result.coverage_percent),
  format!("ORIENTACE: {}",if result.portrait{"NA VYSKU"}else{"NA SIRKU"}),
  "POUZE VYUKA - NE PRO REALIZACI".to_owned()
 ];
 let lx=p.origin_x+p.roof_width+750.;
 let ly=p.origin_y+p.roof_height-100.;
 for (i,line) in legend.iter().enumerate(){
  entities.push(text_at(line,lx,ly-i as f64*360.,
    if i==0{250.}else{180.},"SOL_POPISY"));
 }
 let handles=host.add_entities(entities);
 if let Some(h)=handles.first().filter(|h|!h.is_null()){
  let _=host.write_record(*h,record(vec![
    "type=roof".into(),"schema=solar-cad-cz-v1".into(),
    format!("width_mm={}",p.roof_width),
    format!("height_mm={}",p.roof_height),
    format!("setback_mm={}",p.setback)
  ]));
 }
 for (i,panel) in result.panels.iter().enumerate(){
  if let Some(h)=handles.get(panel_indices[i]).filter(|h|!h.is_null()){
   let _=host.write_record(*h,record(vec![
    "type=module".into(),"schema=solar-cad-cz-v1".into(),
    format!("row={}",panel.row),format!("col={}",panel.column),
    format!("power_wp={}",p.module_watt),
    format!("width_mm={}",p.module_width),
    format!("height_mm={}",p.module_height)
   ]));
  }
 }
 host.end_undo_group();
 host.set_dirty();
 host.push_info(&format!(
  "Solar CAD CZ: {} panelu ({} x {}), {:.3} kWp. POUZE VYUKA.",
  result.panels.len(),result.columns,result.rows,result.total_watt/1000.
 ));
}
fn help(host:&mut dyn HostApi){
 host.push_info("SOL_LAYOUT: Navrh obdelnikove strechy; rozmery jsou v mm.");
 host.push_info("SOL_DEMO: Modelova strecha 10 x 6,5 m, 25 modulu, 11,25 kWp.");
 host.push_info("SOL_HELP: Napoveda. Vytvorene hladiny: SOL_STRECHA, SOL_PANELY, SOL_POPISY.");
 host.push_info("Vyukove odstupy nejsou normove hodnoty. NENI URCENO PRO REALIZACI.");
}
