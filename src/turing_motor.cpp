#include "turing_motor.h"
#include "turing_interface.h"

enum class eval_result{
	cnt,
	err,
	fin,
};

eval_result turing_eval(machine& mc){
	delta d = {mc.state, mc.tape[mc.index]};
	let& rules = mc.rules;
	let& index = mc.index;
	let& tape  = mc.tape;
	if(rules.find(d) == rules.end())
		return eval_result::err;

	let rs = rules[d];
	tape[index] = rs.write;
	switch (rs.mv) {
		left:
			index--;
		break;
		right:
			index++;
		break;
		default:
		break;
	}

	d.state = rs.state;
	d.symbl = tape[index];

	return d.state != mc.acpt? eval_result::cnt: eval_result::fin;
}

eval_result turing(machine mc){
	eval_result res = eval_result::cnt;
	while((res = turing_eval(mc)) == eval_result::cnt);

	return res;
}
