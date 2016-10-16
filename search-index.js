var searchIndex = {};
searchIndex["krpsim"] = {"doc":"# Krpsim","items":[[0,"macros","krpsim","The default module `Macros` adds new macro of krpsim&#39;s library.",null,null],[0,"format","","The module `format` contains the primary module named `Inventory`,\n`Process` and `Optimize`.",null,null],[0,"stock","krpsim::format","the module `stock` describes a list of ressources.",null,null],[0,"ressource","krpsim::format::stock","The module `Ressource` describes a stock.",null,null],[3,"Ressource","krpsim::format::stock::ressource","The `Ressource` structure is the Item implementation.",null,null],[12,"0","","",0,null],[12,"1","","",0,null],[12,"2","","",0,null],[11,"clone","","",0,null],[11,"new","","The `new` constructor function returns the Stock.",0,{"inputs":[{"name":"string"},{"name":"usize"}],"output":{"name":"self"}}],[11,"get_name","","The `get_name` accessor function returns the name\nof ressource.",0,null],[11,"get_quantity","","The `get_quantity` accessor function returns the\nquantity of ressource.",0,null],[11,"get_float_quantity","","The `get_quantity` accessor function returns the\nquantity of ressource.",0,null],[11,"is_empty","","The `is_empty` function return true if quantity is null",0,null],[11,"set_quantity","","The `set_quantity` updates and returns the qte value.",0,null],[11,"add_quantity","","The `add_from_ressource` function additiones a item\nwith a value.",0,null],[11,"add_from_ressource","","The `add_from_ressource` function additiones a item\nwith another item.",0,null],[11,"sub_quantity","","The `sub_quantity` function substrates a item\nwith a value.",0,null],[11,"can_sub","","",0,null],[11,"euclidian_div","","The `euclidian_div` function return the result\nof an euclidian division and set the quantity to the rest",0,null],[11,"sub_from_ressource","","The `sub_from_ressource` function substrates a item\nwith another item.",0,null],[11,"cmp","","The `cmp` function fast compares two Ressource.",0,null],[11,"partial_cmp","","",0,null],[11,"eq","","The `eq` function fast checks if two Ressource are equal.",0,null],[11,"fmt","","The `fmt` function prints the Ressource formated\nlike `&lt;stock_name&gt; :&lt;quantity&gt;`.",0,null],[11,"default","","The `default` constructor function returns a empty Ressource.",0,{"inputs":[],"output":{"name":"self"}}],[11,"sub","","",0,null],[0,"inventory","krpsim::format::stock","The module `Inventory` describes a list of items.",null,null],[3,"Inventory","krpsim::format::stock::inventory","",null,null],[12,"0","","",1,null],[17,"ERR_WRONG_QTE","","",null,null],[17,"ERR_NOT_FOUND","","",null,null],[17,"ERR_EMPTY","","",null,null],[17,"ERR_LESS","","",null,null],[11,"clone","","",1,null],[11,"new","","The `new` constructor function returns the list of ressources.",1,{"inputs":[{"name":"vec"}],"output":{"name":"self"}}],[11,"from_result","","The `from_result` multi constructor function returns a list\nof Item for a list of Result.",1,{"inputs":[{"name":"vec"}],"output":{"name":"option"}}],[11,"print_final","","",1,null],[11,"from_line","","The `from_line` multi constructor function returns a list\nof Item for a need or result argument from `Process`.",1,{"inputs":[{"name":"str"}],"output":{"name":"option"}}],[11,"len","","The `len` interface function returns the number of elements\nin the map.",1,null],[11,"iter","","The `iter` interface function returns a iterator.",1,null],[11,"push","","The `push` interface function inserts a new item to\nthe inventory.",1,null],[11,"is_empty","","The `is_empty` interface function returns true if\nthe map contains not elements.",1,null],[11,"is_zero","","The `is_zero` interface function returns true if\nthe map contains only nul ressources",1,null],[11,"any","","The `any` interface function checks if the map contains\nthe key.",1,null],[11,"any_from_ressource","","The `any` interface function checks if the map contains\nthe key from a ressource.",1,null],[11,"get","","The `get` accessor function returns a item.",1,null],[11,"get_from_ressource","","The `get_from_ressource` accessor function returns a item.",1,null],[11,"get_mut","","The `get_mut` mutator function returns a item.",1,null],[11,"get_mut_from_ressource","","The `get_mut` mutator function returns a item.",1,null],[11,"add","","The `add` interface function additiones a item\nwith another item.",1,null],[11,"add_from_inventory","","The `add` interface function additiones a list of item\nto our self-inventory.",1,null],[11,"sub","","The `sub` interface function substractes a item\nwith another item.",1,null],[11,"sub_from_inventory","","The `sub` interface function substractes a list of item\nto our self-inventory.",1,null],[11,"order","","The `order` takes the payment of command.",1,null],[11,"can_order","","The `can_order` checks if the order is possible.",1,null],[11,"get_ressource","","The `get_ressource` function returns a accessor on\nthe list of ressource.",1,null],[11,"get_neutral","","The `get_neutral` function returns return a neutral component\nif the output ressource exist.",1,null],[11,"to_vec","","The `to_vec` function returns a cloned list of ressource.",1,null],[11,"eq","","The `eq` function fast checks if two Inventory are equal.",1,null],[11,"fmt","","The `fmt` function prints the multiplication list.",1,null],[11,"fmt","","The `fmt` function prints the multiplication list.",1,null],[11,"default","","The `default` constructor function returns a empty Inventory.",1,{"inputs":[],"output":{"name":"self"}}],[0,"operate","krpsim::format","the module `operate` describes a list of process.",null,null],[0,"process","krpsim::format::operate","The module `Process` describes a offer.",null,null],[3,"Process","krpsim::format::operate::process","",null,null],[12,"name","","",2,null],[12,"cycle","","",2,null],[12,"input","","",2,null],[12,"output","","",2,null],[12,"neutral","","",2,null],[12,"heuristic","","",2,null],[17,"ERR_WRONG_CYCLE","","",null,null],[17,"ERR_PARSE","","",null,null],[17,"ERR_BUY","","",null,null],[17,"ERR_NEED","","",null,null],[17,"ERR_REST","","",null,null],[17,"ERR_BOTH","","",null,null],[11,"clone","","",2,null],[11,"new","","The `new` constructor function returns the Process.",2,{"inputs":[{"name":"string"},{"name":"usize"},{"name":"inventory"},{"name":"inventory"},{"name":"hashmap"}],"output":{"name":"self"}}],[11,"from_integer","","",2,{"inputs":[{"name":"string"},{"name":"usize"},{"name":"inventory"},{"name":"inventory"}],"output":{"name":"self"}}],[11,"from_line","","The `from_line` constructor function returns a parsed process.",2,{"inputs":[{"name":"string"},{"name":"str"},{"name":"str"}],"output":{"name":"result"}}],[11,"get_name","","The `get_name` accessor function returns the name\nof process.",2,null],[11,"get_cycle","","The `get_cycle` accessor function returns the number\nof cycle required by the process.",2,null],[11,"buy_with","","The `buy_with` function substrates *with* argument with *input* and\nadditions the *output* to *with* argument.",2,null],[11,"get_h_value","","",2,null],[11,"get_distance","","",2,{"inputs":[{"name":"ressource"},{"name":"vec"}],"output":{"name":"usize"}}],[11,"get_producing_process","","",2,{"inputs":[{"name":"ressource"},{"name":"vec"},{"name":"vec"}],"output":{"name":"vec"}}],[11,"get_ressource_number","","",2,{"inputs":[{"name":"vec"},{"name":"ressource"}],"output":{"name":"f64"}}],[11,"time_cmp","","",2,null],[11,"needed_process","","",2,null],[11,"distance_overall","","",2,null],[11,"fmt","","The `fmt` function prints the Process formated like `&lt;name&gt; :\n(&lt;need&gt; :&lt;qty&gt;[ ;&lt;need&gt; :&lt;qty&gt;[...]]) :\n(&lt;result&gt; :&lt;qty&gt;[ ;&lt;result&gt; :&lt;qty&gt;[...]]) :\n&lt;nb_cycle&gt;`.",2,null],[11,"fmt","","The `fmt` function prints the Process formated like `&lt;name&gt; :\n(&lt;need&gt; :&lt;qty&gt;[ ;&lt;need&gt; :&lt;qty&gt;[...]]) :\n(&lt;result&gt; :&lt;qty&gt;[ ;&lt;result&gt; :&lt;qty&gt;[...]]) :\n&lt;nb_cycle&gt;`.",2,null],[11,"default","","The `default` constructor function returns a empty Proces.",2,{"inputs":[],"output":{"name":"self"}}],[0,"running","krpsim::format::operate","The module `Running` describes a shop.",null,null],[3,"Running","krpsim::format::operate::running","",null,null],[12,"0","","",3,null],[17,"ERR_NOT_FOUND","","",null,null],[17,"ERR_WRONG_CYCLE","","",null,null],[17,"ERR_EMPTY","","",null,null],[11,"new","","The `new` constructor function returns the list of process.",3,{"inputs":[{"name":"vec"}],"output":{"name":"self"}}],[11,"is_empty","","The `is_empty` interface function returns true if\nthe map contains not elements.",3,null],[11,"len","","The `len` interface function returns the number of elements\nin the map.",3,null],[11,"iter","","The `iter` interface function returns a iterator.",3,null],[11,"push","","The `push` interface function inserts a new item to\nthe inventory.",3,null],[11,"get","","The `get_process` accesor function returns a process from map\naccording to a key name.",3,null],[11,"buy_with","","The `buy_with` function buys one command with an inventary.",3,null],[11,"can_cycle","","The `can_cycle` checks if the number and name of cycle is right\nbetween two process.",3,null],[11,"get_process","","The `get_process` function returns a accessor on\nthe list of process.",3,null],[11,"get_cloned_process","","",3,null],[11,"fmt","","The `fmt` function prints the multiplication list.",3,null],[11,"default","","The `default` constructor function returns a empty Running.",3,{"inputs":[],"output":{"name":"self"}}],[0,"optimize","krpsim::format","",null,null],[3,"Optimize","krpsim::format::optimize","The `Optimize` structure is a list of keywords/stock-names who\nhave the priority queue.",null,null],[12,"stock","","",4,null],[12,"time","","",4,null],[11,"new","","The `new` constructor function returns the optimization&#39;s items.",4,{"inputs":[{"name":"vec"},{"name":"bool"}],"output":{"name":"self"}}],[11,"from_line","","The `from_line` constructor function returns the optimization&#39;s item\nfor a parsed line.",4,{"inputs":[{"name":"string"}],"output":{"name":"self"}}],[11,"len","","The `len` interface function returns the number of elements\nin the list.",4,null],[11,"is_empty","","The `is_empty` interface function returns true if\nthe vector Optimize contains no elements.",4,null],[11,"default","","The `default` constructor function returns a empty optimize.",4,{"inputs":[],"output":{"name":"self"}}],[11,"fmt","","The `fmt` function prints the Optimization&#39;s items.",4,null],[0,"livep","krpsim::format","",null,null],[3,"Livep","krpsim::format::livep","",null,null],[12,"process","","",5,null],[12,"cycle_end","","",5,null],[11,"cmp","","",5,null],[11,"eq","","",5,null],[11,"partial_cmp","","",5,null],[11,"new","","",5,{"inputs":[{"name":"process"},{"name":"usize"},{"name":"bool"}],"output":{"name":"self"}}],[11,"destruct","","",5,null],[0,"queue","krpsim::format","",null,null],[3,"Queue","krpsim::format::queue","",null,null],[12,"lst","","",6,null],[11,"new","","",6,{"inputs":[],"output":{"name":"self"}}],[11,"add","","",6,null],[11,"get_ended_process","","",6,null],[11,"is_empty","","",6,null],[0,"parser","krpsim","The module `parser` contains all parssable inputs.",null,null],[0,"config","krpsim::parser","The module `config` describes a configuration of industrial process.",null,null],[3,"Configuration","krpsim::parser::config","The `Configuration` struct contains the Ressource, Process and Optimize.",null,null],[12,"ressources","","",7,null],[12,"running","","",7,null],[12,"optimize","","",7,null],[17,"ERR_READ","","",null,null],[17,"ERR_DOUBLE","","",null,null],[17,"ERR_SPLITN","","",null,null],[11,"new","","The `new` constructor function returns a parsed struct of\nRessource, Process and Optimize.",7,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","The `fmt` function prints the configuration.",7,null],[11,"default","","The `default` constructor function returns a empty configuration.",7,{"inputs":[],"output":{"name":"self"}}],[0,"trace","krpsim::parser","The module `Trace` describes a list of orders.",null,null],[3,"Trace","krpsim::parser::trace","",null,null],[12,"0","","",8,null],[17,"ERR_READ","","",null,null],[17,"ERR_SPLITN","","",null,null],[11,"new","","The `new` constructor function returns a list of order by cycle.",8,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"from_vec","","The `from_vec` constructor function returns a list of order by cycle\naccording to a vector.",8,{"inputs":[{"name":"vec"}],"output":{"name":"self"}}],[11,"iter","","The `iter` interface function returns a iterator.",8,null],[11,"push","","The `push` interface function inserts a new order to\nthe list.",8,null],[11,"fmt","","The `fmt` function prints the list order.",8,null],[11,"default","","The `default` constructor function returns a empty Trace.",8,{"inputs":[],"output":{"name":"self"}}],[14,"println_stderr","krpsim","The `println_stderr` macro writes an error message.",null,null],[14,"from_error","","The `parse_error` macro returns a formated error.",null,null]],"paths":[[3,"Ressource"],[3,"Inventory"],[3,"Process"],[3,"Running"],[3,"Optimize"],[3,"Livep"],[3,"Queue"],[3,"Configuration"],[3,"Trace"]]};
initSearch(searchIndex);