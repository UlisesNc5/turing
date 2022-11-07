#ifndef __TURING_MOTOR_H__
#define __TURING_MOTOR_H__

#include <string>
#include <vector>
#include <map>

#define let auto
#define in :

struct delta{
	std::string state;
	std::string symbl;
};

enum move: char{
	right = '>',
	left  = '<', 
	stay  = '-',
};

struct result{
	std::string state;
	std::string write;
	move mv;
};

typedef std::map<delta, result>  tran_rules;
typedef std::vector<std::string> mem_tape;

struct machine{
	tran_rules rules;
	mem_tape   tape;
	size_t     index;
	std::string state;

	std::string acpt;
	std::string init;
	std::string name;
};

#endif
