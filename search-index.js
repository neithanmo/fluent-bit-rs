var N=null,E="",T="t",U="u",searchIndex={};
var R=["fluentbit","FLB_ERROR","FLB_RETRY","flbresult","result","try_from","borrow","type_id","typeid","borrow_mut","try_into","plugininfo","PluginInfo","FLBError","FLBPluginMethods"];

searchIndex[R[0]]={"doc":"fluentbit This crate aims to build output plugins for…","i":[[3,R[12],R[0],"Basic plugin information",N,N],[12,"name",E,"Plugin's name",0,N],[12,"description",E,"Plugin's description",0,N],[4,R[13],E,"Fluent-bit error definitions",N,N],[13,R[1],E,"The data have been processed normally.",1,N],[13,R[2],E,"A recoverable error have ocurred, the engine can try to…",1,N],[6,"FLBResult",E,"Custom result for any plugin's operation",N,N],[17,R[1],E,E,N,N],[17,"FLB_OK",E,E,N,N],[17,R[2],E,E,N,N],[17,"FLB_PROXY_OUTPUT_PLUGIN",E,E,N,N],[17,"FLB_PROXY_GOLANG",E,E,N,N],[8,R[14],E,"Trait which defines the functions that should be…",N,N],[10,"plugin_register",E,"A plugin register method",2,[[["self"],[R[11]]],[R[3]]]],[10,"plugin_init",E,"Before the engine starts, it initialize all plugins that…",2,[[["self"]],[R[3]]]],[10,"plugin_flush",E,"Upon flush time, when Fluent Bit want's to flush it…",2,N],[10,"plugin_exit",E,"When Fluent Bit will stop using the instance of the…",2,[[["self"]],[R[3]]]],[14,"create_boilerplate",E,"This macro will generate the needed boilerplate for output…",N,N],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[5],E,E,0,[[[U]],[R[4]]]],[11,R[6],E,E,0,[[["self"]],[T]]],[11,R[7],E,E,0,[[["self"]],[R[8]]]],[11,R[9],E,E,0,[[["self"]],[T]]],[11,R[10],E,E,0,[[["self"]],[R[4]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[5],E,E,1,[[[U]],[R[4]]]],[11,R[6],E,E,1,[[["self"]],[T]]],[11,R[7],E,E,1,[[["self"]],[R[8]]]],[11,R[9],E,E,1,[[["self"]],[T]]],[11,R[10],E,E,1,[[["self"]],[R[4]]]],[11,"default",E,E,0,[[],[R[11]]]]],"p":[[3,R[12]],[4,R[13]],[8,R[14]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);