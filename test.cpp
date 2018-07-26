#include <stdio.h>
#include <stdlib.h>
#include <iostream>
#include <vector>
#include <chrono>
#include <sched.h>
#include <atomic>

      

#define PAR 1
#define DATASIZE 524288

std::vector<std::vector<int>> output;
std::vector<int> input;


int sleep_print1(int task) {
       
    int l = DATASIZE/PAR;
    int off = task*(DATASIZE/PAR);
    auto temp = &output[task][0];
    auto ip = &input[off];
    for(int i=0;i<l;++i){
        *temp=*ip;//+off;
        temp+=1;
        ip+=1;
    }
	return 0;
}


int sleep_print2(int task) {
    auto& temp = output[task];
    auto temp_p = &output[task][0];
    auto temp_p2 = temp_p + DATASIZE/PAR;
    int expected = task*(DATASIZE/PAR);
    while(temp_p!=temp_p2) {
        if (*temp_p!=expected)
            printf("Woha!\n");
        temp_p+=1;
        expected+=1;
    }
	return 0;
}

std::atomic_int valsync=0;
std::atomic_int valdone=0;

void* threadfunc(void* p) {
    int i = (int)(long)p;
    cpu_set_t set;
    CPU_ZERO(&set);
    CPU_SET(i, &set);
        sched_setaffinity(0, sizeof(set),&set);
    
    int expect=1;
    while(true) {
        while(valsync.load()!=expect) {
        }
        expect+=1;        
        sleep_print1(i);
        valdone+=1;
        
        while(valsync.load()!=expect) {
        }
        expect+=1;        
        sleep_print2(i);    
        valdone+=1;
    }
    
}

int main() {

    for(int i=0;i<DATASIZE;++i) {
        input.push_back(i);
    }
    for(int i=0;i<PAR;++i) {
        std::vector<int> t;
        for(int j=0;j<DATASIZE/PAR;++j)
            t.push_back(0);
        output.push_back(t);
    }
	for (int i = 0; i < PAR ; ++i)
	{
	    pthread_t thread_id;
        if(pthread_create(&thread_id, NULL, threadfunc, (void*)i)) {

            fprintf(stderr, "Error creating thread\n");
            return 1;

        }	
	}
	for(int run=0;run<20;++run)
	{
        std::chrono::steady_clock::time_point t1 = std::chrono::steady_clock::now();
	    for(int j=0;j<1000;++j) {

            std::atomic_fetch_add(&valsync,1);
            while(true)  {
                int expected=PAR;
                if (std::atomic_compare_exchange_strong(&valdone,&expected,0))
                    break;

            }


            std::atomic_fetch_add(&valsync,1);
            while(true)  {
                int expected=PAR;
                if (std::atomic_compare_exchange_strong(&valdone,&expected,0))
                    break;
            }
	    }
	    std::chrono::steady_clock::time_point t2= std::chrono::steady_clock::now();	
	    auto delta  = t2-t1;
	            
	    std::cout<<"Time: "<<std::chrono::duration_cast<std::chrono::nanoseconds>(delta).count()/1000<<" ns per iter \n";
	}
	
	return 0;
}

