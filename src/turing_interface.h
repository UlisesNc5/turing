#ifndef __TURING_INTERFACE_H__
#define __TURING_INTERFACE_H__

#ifndef __cpp
extern "C"{
#endif
typedef enum ceval{
	cnt,
	err,
	fin,
}ceval;

typedef struct cstate{
	char* ptr;
	unsigned size;
} cstate;

typedef struct ctape{
	char** ptr;
	unsigned size;
} ctape;

ctape  turing_get_tape();
cstate turing_get_state();
ceval  turing_next();
ceval  turing_instant();
void  compile(const char* path);
void  turing_init();

#ifndef __cpp
}
#endif
#endif
