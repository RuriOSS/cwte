# WIP:
Nothoing implemented yet, just some design ideas.
# About CE-lang:
Just a cute error handling extension for C, with no syntax breaking, and the tail will never wag the cat.
We will just have a new happy face `:>` for default handling, and a sad face `:<` for error handling, and `#[[ce_foo()]]` for code generation.     
These syntax will be translated to C code, you can use CE-lang for error handling, CE-generator transform it to C, and you compile/run/debug the generated C code.      
# Why CE-lang:
In ruri:      
```c
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept4), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(access), 0);
ruri_check_seccomp_ret(res, container->no_warnings);
```
Too ugly you see.     
seccomp_rule_add() uses va_args, so if you don't use these complex code, you can only use a macro. But in cross-arch ci, it will bomb to TLE, as the pre-compile expansion performance of macro is not good, and qemu is slow.      
So, I want a:     
```ce
#[[ce_reg(seccomp_rule_add, int, _<0)]]
seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0) :<;
res = seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept4), 0) :<;
seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(access), 0) :<;
```
It's better, right?    
And it will be auto expanded to code like this:    
```c
if(seccomp_rule_add(ctx, SCMP_ACT_ALLOW, SCMP_SYS(accept), 0) != 0) {
    warning("seccomp_rule_add", __FILE__, __LINE__, res, errno);
}
```
So that's CE lang, C with better Error handling/Cute Error handling.    
The tail will never wag the cat.    
So ce-lang will never break c syntax, except the old `:>` as `]` design.   
But as ce will translate .ce to c, and if you only use `:>` as happy face in .ce, that's fine.    
In one word, CE-lang makes a zipped error handling in C, and it's kawaii.      
# The .hce header:
.hce stands for `happy c ending`, it's just a kv-map to register error expr and handler for funcs. maybe we can also have standard hce conf like posix.hce.      
```ce
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

# CE-lang design goals:
```ce
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
```

# Note:
CE-lang has no super cow powers.    
The tail will never wag the cat.    
CE-lang is just for zipping complex unhappy path logic, and make it more readable.    
CE-lang just implements `:<` and `:>`, `and #[[ce_foo()]]`, the rest is just C code, and every CE-lang feature will be translated to C code, You debug/run the generated C code, not the CE-lang code.   
#[[ce_reg()]] is enforced, or ce will not know how to handle the error.      
In one word, CE-lang is like yet another C-style .unwrap().    
And, there will be many ubs, so always do a diff-check between .ce and .c, and make sure the generated code is what you want.    
You can use _CE_DFT for `:>` and _CE_PAN for `:<`, just recover with one `sed`, so your IDE and clang-format will not scream at it. But for `foo() :<, :>`, your IDE will scream anyway, although these code are less in real-world case.    