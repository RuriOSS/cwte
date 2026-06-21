# WIP:
>[!WARNING]
>Nothoing implemented yet, just some design ideas.    

But, if you throw this doc to LLM, let LLM refine it to spec, and use LLM as your ceg, you have cwte right now.    
"I'm a lazy dev, and I used :< sad face to mark the code that mignt fail, as my assistant, you should implement the :< mark as error handling logic for these code".   
And, the real cwte-generator will just be a pre-compile code generator for ruri. It will not act on other unnecessary features that I will not use it in my code.     
# Version:
0.0, just a draft, not a spec or implementation yet.     
# About cwte:
Cwte(cute) stands for "C with tailed error-handler/Cute way to handle error/~~cry to error~~", it's a cute and concise error handling extension for C, with zero syntax breaking, and the tail will never wag the cat.     
Just a cute error handling extension for C.    
With no syntax breaking, and the tail will never wag the cat.    
We will just have a new happy face `:>` for default handling, and a sad face `:<` for error handling, and `#[[ce_foo()]]` for code generation.      
These syntax will be translated to C code, you can use cwte for error handling, cwte-generator transform it to C, and you compile/run/debug the generated C code.      
In short, cwte is just for zipping complex unhappy path logic, and make it more readable.    
I just hope it can save some time, so we can have an ice cream.    
# The core:
`:<` Is the only core feature, it's a tail after func call, for error handling.     
The tail should never wag the cat, this means sad path handler should never pollute the core logic, and cwte will also never pollute other c code.     
The tail should never wag the cat also means `tail` command should not call `|cat` :D   
# The philosophy:
- Cwte has no super cow powers.    
- Cwte is a postfix, a tail, but not the cat (C-lang).    
- The tail can/should/will/must never wag the cat.
- Cwte should be reversible, if you don't like, thow it away and fallback to c.
- We are c users, not cwte users.    
- Cwte is dangerous, the tail can make everything cooked, so:
  - Always check the generated code.
  - Always make a backup to last working code.
  - Always backup ceg itself.
  - The more cwte you use, the longer tail you have.   
  - Always check where's your cat.
  - If cwte wrote the wrong code, the dev should scream. 
  - Cwte should NEVER be a compile-time dependency for released code.
  - Never assume anything, your cat's tail can make your cook fail.    

# Project scructure:
A cwte project should be like:
```
project
├── src // For C code, the ONLY code for testing and publishing.
│   └── foo.c // The ONLY code as true source code, for testing and publishing.
└── srce // For cwte code, only TEMPORARY code for developing.
    ├── foo.ce // Cwte code. For reading and developing.
    └── foo.hce // Cwte defination, for registering func type and handler.
```

# Why cwte:
In ruri:      
```c
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept4), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(access), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
```
Too ugly you see? I scream, eye scream, my small screen scream, my ADHD scream, my LLM scream, all scream.         
seccomp_rule_add() uses va_args, so if you don't use these complex code, you can only use a macro. But in cross-arch ci, it will bomb to TLE, as the pre-compile expansion performance of macro is not good, and qemu is slow.      
So, I want a:     
```c
#[[ce_reg(seccomp_rule_add, int, _<0)]]
```
Then:   
```c
seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0) :<;
seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept4), 0) :<;
seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(access), 0) :<;
```
Now we have ice cream, but no more `I scream`.   
Dev happy, reader happy, PRs happy, LLM happy (with prompt), all happy.   
It will also be very useful in educational case, as you can use a `:<` to tell people "you should handle this error, but it's not the core logic for our code", and your example code will be more concise and readable.    
And the above code will be auto expanded to code like this:    
```c
if(seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0) != 0) {
    warning("seccomp_rule_add", __FILE__, __LINE__, res, errno);
}
```
So that's cwte, a cute tail.    
The tail will never wag the cat.    
So cwte will never break c syntax, except the old `:>` as `]` design.   
But as cwte will translate .ce to c, and if you only use `:>` as happy face in .ce, that's fine.    
In one word, cwte makes a zipped error handling in C, and it's kawaii.      
# Why sad face `:<`:
- Cute and readable, it's like a sad face, and it zips the error handling logic, and make the code more concise and readable.    
- Zero syntax breaking, :< never affects C grammar.    
- Explicit invalid-stat marker, it's illegal, if you leave a `:<`, `:>` or `]` after a function, your compiler will definitely scream.    
- Enforced pre-compile code generation and error handling, `:<` is not a todo comment, but it's easier to be done. With LLM or ceg.    
# The .hce header:
.hce stands for `happy c ending/handle c error`, it's just a kv-map to register error expr and handler for funcs. maybe we can also have standard hce conf like posix.hce.      
```c
// Register function type and failure condition
#[[ce_reg(func, type, exp)]]
// For example:
#[[ce_reg(open, int, _<0)]]

// Register function's panic and default handlers
#[[ce_pan(func, panic)]]
#[[ce_dft(func, def)]]
// For example:
#[[ce_pan(open, panic)]]
#[[ce_dft(open, log)]]
```

.hce shoud only contain the three simple commands, and other definations, like `#define panic()`, `#define log()`, and `typedef` should be in .ce or your .h, as .hce is just `happy c ending/handle c error` delclaration file.    
# cwte design goals:
```c
// Will call panic() if open returns < 0
int fd = open("file.txt", O_RDONLY) :<;

// Will call panic if open returns < 0,
// and call log() if open returns >= 0
int fd_2 = open("file2.txt", O_RDONLY) :<, :>;

// Will call user defined panic and log logic.
int fd_3 = open("file3.txt", O_RDONLY) :<
{
	printf("Panic in open with file3.txt\n");
	exit(1);
}
:>
{
	printf("Log in open with file3.txt\n");
}

// Will call user defined panic logic, and default to log if not panic.
int fd_4 = open("file4.txt", O_RDONLY) :<
{
	printf("Panic in open with file4.txt\n");
	exit(1);
}
:>;

// Just add a default log handler for open, will be triggered even fail.
int fd_5 = open("file5.txt", O_RDONLY) :>;
```

# Note:  
cwte just implements `:<` and `:>`, `and #[[ce_foo()]]`, the rest is just C code, and every cwte feature will be translated to C code, You debug/run the generated C code, not the cwte code.   
#[[ce_reg()]] is enforced, or ce will not know how to handle the error.        
And, there will be many ubs, so always do a diff-check between .ce and .c, and make sure the generated code is what you want.    
You can use _CE_DFT for `:>` and _CE_PAN for `:<`, just recover with one `sed`, so your IDE and clang-format will not scream at it. But for `foo() :<, :>`, your IDE will scream anyway, although these code are less in real-world case.    
Cwte should be used step-by-step, and always check the generated code to make sure it's what you want. If it will be more ugly, immediately make a ctrl-z in your ide and rollback to the c way, we should never let the tail wag the cat.    

cwte will use line-no for internal variable name, so you will match generited code with .ce easily.    
# Future:
Maybe we can have a `#[[ce_enforce(func)]]` to enforce you catch result for func in cwte, and `:D` for ignoring the error, and `:o` for only log when error, `:~ { ... }` for a custom handler, and even `::}` to output a nautilus in ceg, and use `::}` as a readable todo note.        
Maybe one day it can be C-Way-To-Evolve, but at least these ideas shows that c is extensible, and cwte is also.    
Cwte never assumes it won't become a fossil.   
"But if we have to evolve, are there a trackable way?"    
# Ascii logo:
```
         _-''''-._
       /`          `. 
      /   .'~~~,     \ 
     |   /       \    |    
     |   |   :>.,/    |
     \   '.       ,___/~~~
     `.   '-----`'  /~~~~~~
       `.          /~~~~~~~~
         '-.____. /~~~~~~~~~~~
```