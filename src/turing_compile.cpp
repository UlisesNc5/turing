#include <algorithm>
#include <fstream>
#include "turing_motor.h"

static std::vector<std::string> parse_file(const char* path){
	let file = std::ifstream(path);
	let raw_st = std::string("");
	let cok_st = std::string("");
	if(!file.is_open())
		return std::vector<std::string>(); 

	while(!file.eof()){
		char val = 0;
		file.read(&val, 1);
		raw_st += val;
	}

	//remove comments and whitespace
	let size = raw_st.size();
	raw_st.push_back('\n');
	for(size_t i = 0; i < size; ++i){
		char c = raw_st[i + 0];
		char n = raw_st[i + 1];
		if(c == '/' && n == '/'){
			while(i < size && c != '\n')
				c = raw_st[++i];
			c = '\n';
		}
		if(c == '/' && n == '*'){
			do{
				i++;
				c = raw_st[i + 0];
				n = raw_st[i + 1];
			}while(i < size &&(c != '*' || n != '/'));
			i += 2;
			c = '\n';
		}
		if(c == ' ') continue;
		cok_st.push_back(c);
	}

	//divide into statements
	std::vector<std::string> stnc = {""};
	for(auto& letter : cok_st){
		//std::cout << letter ;
		if(letter == '\n'){
			if(!stnc.back().empty()){
				stnc.push_back("");
			}
			//std::cout << "ENDLINE\n";
		}
		else
			stnc.back().push_back(letter);
	}
	while(stnc.back()[0] == 0)
		stnc.pop_back();
	return stnc;
}

bool operator==(const delta this_, const delta& other){
	return this_.state == other.state && this_.symbl == other.symbl;
}

bool operator<(const delta this_, const delta& other){
	return this_.state < other.state && this_.symbl < other.symbl;
}

std::vector<std::string> separate(std::string& strg, char sep = ','){
	std::vector<std::string> vs = {""};

	size_t i = 0;
	while(i < strg.size()){
		for(; i < strg.size(); ++i)
			vs.back().push_back(strg[i]);
		vs.push_back("");
	}
	
	return vs;
}

void add_rule(tran_rules& map, std::string d, std::string r){
	delta rd = {"", ""};
	result rr = {"", "", move::stay};

	let d_sep = separate(d);
	rd.state = std::move(d_sep[0]);
	rd.symbl = std::move(d_sep[1]);

	if(map.find(rd) == map.end())
		return;

	let r_sep = separate(r);
	if(r_sep[2][0] != 0)
		return;
	rr.state = std::move(r_sep[0]);
	rr.write = std::move(r_sep[1]);
	rr.mv = (move)r_sep[2][0];
	
	map[rd] = rr;
}

machine mc = {};

void compile(const char* path){
	let statements =parse_file("test.tr");

	for(size_t i = 3; i < statements.size() - 3; i += 2){
		add_rule(mc.rules, std::move(statements[i + 0]), std::move(statements[i + 1]));
	}

	//extremely shitty way to implement this lmao
	mc.name = &statements[0][0] + 5; 
	mc.init = &statements[1][0] + 5; 
	mc.acpt = &statements[2][0] + 7;
}
