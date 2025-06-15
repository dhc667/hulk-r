@.fstr = private constant [4 x i8] c"%f\0A\00", align 1
@.true_str = private constant [6 x i8] c"true\0A\00", align 1
@.false_str = private constant [7 x i8] c"false\0A\00", align 1
@.none_str = private constant [6 x i8] c"none\0A\00", align 1
declare i32 @printf(i8*, ...)
declare i32 @sprintf(i8*, i8*, ...)
declare i8* @strcat(i8*, i8*)
declare i8* @strcpy(i8*, i8*)
declare i32 @strlen(i8*)
declare i32 @strcmp(i8*, i8*)
declare i8* @malloc(i64)
@.fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
define %Object_type* @id(%Object_type* %x) {
entry:
%tmp0 = alloca i8*, align 8
store i8* %x, i8** %tmp0, align 8
%tmp1 = load i8*, i8** %tmp0, align 8
  ret %Object_type* %tmp1
}

define i32 @main() {
entry:

ret i32 0
}
