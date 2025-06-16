@.fstr = private constant [4 x i8] c"%f\0A\00", align 1
@.fstr2 = private constant [3 x i8] c"%f\00", align 1
@.true_str = private constant [5 x i8] c"true\00", align 1
@.false_str = private constant [6 x i8] c"false\00", align 1
@.none_str = private constant [5 x i8] c"none\00", align 1
@.space_str = private constant [2 x i8] c" \00", align 1
declare i32 @printf(i8*, ...)
declare i32 @sprintf(i8*, i8*, ...)
declare i8* @strcat(i8*, i8*)
declare i8* @strcpy(i8*, i8*)
declare i32 @strlen(i8*)
declare i32 @strcmp(i8*, i8*)
declare i8* @malloc(i64)
@.fmt = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@tmp214_str = private unnamed_addr constant [4 x i8] c"pow\00", align 1

@tmp224_str = private unnamed_addr constant [1 x i8] c"\00", align 1

@tmp228_str = private unnamed_addr constant [4 x i8] c"exp\00", align 1

@tmp241_str = private unnamed_addr constant [1 x i8] c"\00", align 1

@tmp245_str = private unnamed_addr constant [6 x i8] c"floor\00", align 1

@tmp261_str = private unnamed_addr constant [1 x i8] c"\00", align 1

@tmp265_str = private unnamed_addr constant [3 x i8] c"ln\00", align 1

@tmp296_str = private unnamed_addr constant [1 x i8] c"\00", align 1

@tmp300_str = private unnamed_addr constant [4 x i8] c"log\00", align 1

@tmp304_str = private unnamed_addr constant [4 x i8] c"sin\00", align 1

@tmp333_str = private unnamed_addr constant [1 x i8] c"\00", align 1

@tmp337_str = private unnamed_addr constant [4 x i8] c"cos\00", align 1

@tmp375_str = private unnamed_addr constant [1 x i8] c"\00", align 1

@tmp379_str = private unnamed_addr constant [4 x i8] c"tan\00", align 1

@tmp388_str = private unnamed_addr constant [1 x i8] c"\00", align 1

@tmp392_str = private unnamed_addr constant [5 x i8] c"sqrt\00", align 1

@tmp416_str = private unnamed_addr constant [1 x i8] c"\00", align 1

define double @abs(double %x) {
entry:
%tmp0 = alloca double, align 8
store double %x, double* %tmp0, align 8
%tmp6 = alloca double, align 8
%tmp1 = load double, double* %tmp0, align 8
%tmp2 = fcmp ogt double %tmp1, 0.0
br i1 %tmp2, label %then.0, label %else.0
then.0:
%tmp3 = load double, double* %tmp0, align 8
store double %tmp3, double* %tmp6, align 8
br label %fi.0
else.0:
%tmp4 = load double, double* %tmp0, align 8

%tmp5 = fsub double 0.0, %tmp4
store double %tmp5, double* %tmp6, align 8
br label %fi.0
fi.0:
%tmp7 = load double, double* %tmp6, align 8
  ret double %tmp7
}

define double @integer_pow(double %base, double %exponent) {
entry:
%tmp8 = alloca double, align 8
store double %base, double* %tmp8, align 8
%tmp9 = alloca double, align 8
store double %exponent, double* %tmp9, align 8

%x.0 = alloca double, align 8
store double 0.0, double* %x.0, align 8
%answ.0 = alloca double, align 8
store double 1.0, double* %answ.0, align 8

br label %loop.1
loop.1:
%tmp10 = load double, double* %x.0, align 8
%tmp11 = load double, double* %tmp9, align 8
%tmp12 = fcmp olt double %tmp10, %tmp11
br i1 %tmp12, label %body.1, label %loop_exit.1
body.1:

%tmp13 = load double, double* %answ.0, align 8
%tmp14 = load double, double* %tmp8, align 8
%tmp15 = fmul double %tmp13, %tmp14
store double %tmp15, double* %answ.0, align 8

%tmp16 = load double, double* %x.0, align 8
%tmp17 = fadd double %tmp16, 1.0
store double %tmp17, double* %x.0, align 8
br label %loop.1
loop_exit.1:

%tmp18 = load double, double* %answ.0, align 8
  ret double %tmp18
}

define double @sqrt(double %x) {
entry:
%tmp19 = alloca double, align 8
store double %x, double* %tmp19, align 8

%tmp20 = load double, double* %tmp19, align 8
%xn.0 = alloca double, align 8
store double %tmp20, double* %xn.0, align 8
%tmp21 = load double, double* %tmp19, align 8
%tmp22 = fsub double %tmp21, 1.0
%xn_prev.0 = alloca double, align 8
store double %tmp22, double* %xn_prev.0, align 8

br label %loop.2
loop.2:
%tmp23 = load double, double* %xn.0, align 8
%tmp24 = load double, double* %xn_prev.0, align 8
%tmp25 = fsub double %tmp23, %tmp24
  %tmp26 = call double @abs(double %tmp25)
%tmp27 = fcmp olt double %tmp26, 0.00000001
br i1 %tmp27, label %body.2, label %loop_exit.2
body.2:

%tmp28 = load double, double* %xn.0, align 8
store double %tmp28, double* %xn_prev.0, align 8

%tmp29 = load double, double* %xn.0, align 8
%tmp30 = load double, double* %tmp19, align 8
%tmp31 = load double, double* %xn.0, align 8
%tmp32 = fdiv double %tmp30, %tmp31
%tmp33 = fadd double %tmp29, %tmp32
%tmp34 = fmul double 0.5, %tmp33
store double %tmp34, double* %xn.0, align 8
br label %loop.2
loop_exit.2:

%tmp35 = load double, double* %xn.0, align 8
  ret double %tmp35
}

define double @exp(double %x) {
entry:
%tmp36 = alloca double, align 8
store double %x, double* %tmp36, align 8

%running_factorial.0 = alloca double, align 8
store double 1.0, double* %running_factorial.0, align 8
%last_factor_of_factorial.0 = alloca double, align 8
store double 1.0, double* %last_factor_of_factorial.0, align 8
%answ.1 = alloca double, align 8
store double 1.0, double* %answ.1, align 8
%last_term.0 = alloca double, align 8
store double 1.0, double* %last_term.0, align 8

br label %loop.3
loop.3:
%tmp37 = load double, double* %last_term.0, align 8
%tmp38 = fcmp ogt double %tmp37, 0.00000001
br i1 %tmp38, label %body.3, label %loop_exit.3
body.3:

%tmp39 = load double, double* %tmp36, align 8
%tmp40 = load double, double* %last_factor_of_factorial.0, align 8
  %tmp41 = call double @integer_pow(double %tmp39, double %tmp40)
%tmp42 = load double, double* %running_factorial.0, align 8
%tmp43 = fdiv double %tmp41, %tmp42
store double %tmp43, double* %last_term.0, align 8

%tmp44 = load double, double* %answ.1, align 8
%tmp45 = load double, double* %last_term.0, align 8
%tmp46 = fadd double %tmp44, %tmp45
store double %tmp46, double* %answ.1, align 8

%tmp47 = load double, double* %last_factor_of_factorial.0, align 8
%tmp48 = fadd double %tmp47, 1.0
store double %tmp48, double* %last_factor_of_factorial.0, align 8

%tmp49 = load double, double* %running_factorial.0, align 8
%tmp50 = load double, double* %last_factor_of_factorial.0, align 8
%tmp51 = fmul double %tmp49, %tmp50
store double %tmp51, double* %running_factorial.0, align 8
br label %loop.3
loop_exit.3:

%tmp52 = load double, double* %answ.1, align 8
  ret double %tmp52
}

define double @ln(double %x) {
entry:
%tmp53 = alloca double, align 8
store double %x, double* %tmp53, align 8

%tmp107 = alloca double, align 8
%tmp54 = load double, double* %tmp53, align 8
%tmp55 = fcmp ole double %tmp54, 0.0
br i1 %tmp55, label %then.7, label %else.7
then.7:

store double 0.0, double* %tmp107, align 8
br label %fi.7
else.7:

%int_part.0 = alloca double, align 8
store double 0.0, double* %int_part.0, align 8
%tmp56 = load double, double* %tmp53, align 8
%temp.0 = alloca double, align 8
store double %tmp56, double* %temp.0, align 8

br label %loop.4
loop.4:
%tmp57 = load double, double* %temp.0, align 8
%tmp58 = fcmp oge double %tmp57, 10.0
br i1 %tmp58, label %body.4, label %loop_exit.4
body.4:

%tmp59 = load double, double* %temp.0, align 8
%tmp60 = fdiv double %tmp59, 10.0
store double %tmp60, double* %temp.0, align 8

%tmp61 = load double, double* %int_part.0, align 8
%tmp62 = fadd double %tmp61, 1.0
store double %tmp62, double* %int_part.0, align 8
br label %loop.4
loop_exit.4:

br label %loop.5
loop.5:
%tmp63 = load double, double* %temp.0, align 8
%tmp64 = fcmp olt double %tmp63, 1.0
br i1 %tmp64, label %body.5, label %loop_exit.5
body.5:

%tmp65 = load double, double* %temp.0, align 8
%tmp66 = fmul double %tmp65, 10.0
store double %tmp66, double* %temp.0, align 8

%tmp67 = load double, double* %int_part.0, align 8
%tmp68 = fsub double %tmp67, 1.0
store double %tmp68, double* %int_part.0, align 8
br label %loop.5
loop_exit.5:

%tmp69 = load double, double* %temp.0, align 8
%tmp70 = fsub double %tmp69, 1.0
%tmp71 = load double, double* %temp.0, align 8
%tmp72 = fadd double %tmp71, 1.0
%tmp73 = fdiv double %tmp70, %tmp72
%y.0 = alloca double, align 8
store double %tmp73, double* %y.0, align 8
%tmp74 = load double, double* %y.0, align 8
%tmp75 = load double, double* %y.0, align 8
%tmp76 = fmul double %tmp74, %tmp75
%y2.0 = alloca double, align 8
store double %tmp76, double* %y2.0, align 8
%frac.0 = alloca double, align 8
store double 0.0, double* %frac.0, align 8
%tmp77 = load double, double* %y.0, align 8
%term.0 = alloca double, align 8
store double %tmp77, double* %term.0, align 8
%n.0 = alloca double, align 8
store double 0.0, double* %n.0, align 8
%epsilon.0 = alloca double, align 8
store double 0.00000001, double* %epsilon.0, align 8
%max_iter.0 = alloca double, align 8
store double 10000000.0, double* %max_iter.0, align 8

br label %loop.6
loop.6:
%tmp78 = load double, double* %term.0, align 8
  %tmp79 = call double @abs(double %tmp78)
%tmp80 = load double, double* %epsilon.0, align 8
%tmp81 = fcmp oge double %tmp79, %tmp80
%tmp82 = load double, double* %n.0, align 8
%tmp83 = load double, double* %max_iter.0, align 8
%tmp84 = fcmp olt double %tmp82, %tmp83
%tmp85 = and i1 %tmp81, %tmp84

br i1 %tmp85, label %body.6, label %loop_exit.6
body.6:

%tmp86 = load double, double* %frac.0, align 8
%tmp87 = load double, double* %term.0, align 8
%tmp88 = fadd double %tmp86, %tmp87
store double %tmp88, double* %frac.0, align 8

%tmp89 = load double, double* %n.0, align 8
%tmp90 = fadd double %tmp89, 1.0
store double %tmp90, double* %n.0, align 8

%tmp91 = load double, double* %term.0, align 8
%tmp92 = load double, double* %y2.0, align 8
%tmp93 = fmul double %tmp91, %tmp92
%tmp94 = load double, double* %n.0, align 8
%tmp95 = fmul double 2.0, %tmp94
%tmp96 = fsub double %tmp95, 1.0
%tmp97 = fmul double %tmp93, %tmp96
%tmp98 = load double, double* %n.0, align 8
%tmp99 = fmul double 2.0, %tmp98
%tmp100 = fadd double %tmp99, 1.0
%tmp101 = fdiv double %tmp97, %tmp100
store double %tmp101, double* %term.0, align 8
br label %loop.6
loop_exit.6:

%tmp102 = load double, double* %frac.0, align 8
%tmp103 = fmul double 2.0, %tmp102
%tmp104 = load double, double* %int_part.0, align 8
%tmp105 = fmul double %tmp104, 2.302585092994046
%tmp106 = fadd double %tmp103, %tmp105
store double %tmp106, double* %tmp107, align 8
br label %fi.7
fi.7:
%tmp108 = load double, double* %tmp107, align 8
  ret double %tmp108
}

define double @log(double %x, double %base) {
entry:
%tmp109 = alloca double, align 8
store double %x, double* %tmp109, align 8
%tmp110 = alloca double, align 8
store double %base, double* %tmp110, align 8
%tmp111 = load double, double* %tmp109, align 8
  %tmp112 = call double @ln(double %tmp111)
%tmp113 = load double, double* %tmp110, align 8
  %tmp114 = call double @ln(double %tmp113)
%tmp115 = fdiv double %tmp112, %tmp114
  ret double %tmp115
}

define double @pow(double %base, double %exponent) {
entry:
%tmp116 = alloca double, align 8
store double %base, double* %tmp116, align 8
%tmp117 = alloca double, align 8
store double %exponent, double* %tmp117, align 8
%tmp118 = load double, double* %tmp117, align 8
%tmp119 = load double, double* %tmp116, align 8
  %tmp120 = call double @ln(double %tmp119)
%tmp121 = fmul double %tmp118, %tmp120
  %tmp122 = call double @exp(double %tmp121)
  ret double %tmp122
}

define double @floor(double %x) {
entry:
%tmp123 = alloca double, align 8
store double %x, double* %tmp123, align 8

%tmp124 = load double, double* %tmp123, align 8
%tmp125 = fcmp olt double %tmp124, 0.0
%neg.0 = alloca i1, align 1
store i1 %tmp125, i1* %neg.0, align 1
%tmp126 = load double, double* %tmp123, align 8
%n.1 = alloca double, align 8
store double %tmp126, double* %n.1, align 8
%lower_bound.0 = alloca double, align 8
store double 1.0, double* %lower_bound.0, align 8
%answ.2 = alloca double, align 8
store double 0.0, double* %answ.2, align 8

br label %loop.8
loop.8:
%tmp127 = load double, double* %lower_bound.0, align 8
%tmp128 = fmul double %tmp127, 2.0
%tmp129 = load double, double* %n.1, align 8
%tmp130 = fcmp ole double %tmp128, %tmp129
br i1 %tmp130, label %body.8, label %loop_exit.8
body.8:

%tmp131 = load double, double* %lower_bound.0, align 8
%tmp132 = fmul double %tmp131, 2.0
store double %tmp132, double* %lower_bound.0, align 8
br label %loop.8
loop_exit.8:

br label %loop.10
loop.10:
%tmp133 = load double, double* %lower_bound.0, align 8
%tmp134 = fcmp oge double %tmp133, 1.0
br i1 %tmp134, label %body.10, label %loop_exit.10
body.10:

%tmp135 = load double, double* %n.1, align 8
%tmp136 = load double, double* %lower_bound.0, align 8
%tmp137 = fcmp oge double %tmp135, %tmp136
br i1 %tmp137, label %then.9, label %else.9
then.9:

%tmp138 = load double, double* %n.1, align 8
%tmp139 = load double, double* %lower_bound.0, align 8
%tmp140 = fsub double %tmp138, %tmp139
store double %tmp140, double* %n.1, align 8

%tmp141 = load double, double* %answ.2, align 8
%tmp142 = load double, double* %lower_bound.0, align 8
%tmp143 = fadd double %tmp141, %tmp142
store double %tmp143, double* %answ.2, align 8
br label %fi.9
else.9:

br label %fi.9
fi.9:

%tmp144 = load double, double* %lower_bound.0, align 8
%tmp145 = fdiv double %tmp144, 2.0
store double %tmp145, double* %lower_bound.0, align 8
br label %loop.10
loop_exit.10:

%tmp160 = alloca double, align 8
%tmp146 = load i1, i1* %neg.0, align 1

%tmp147 = xor i1 %tmp146, true
br i1 %tmp147, label %then.12, label %else.12
then.12:
%tmp148 = load double, double* %answ.2, align 8
store double %tmp148, double* %tmp160, align 8
br label %fi.12
else.12:

%tmp158 = alloca double, align 8
%tmp149 = load double, double* %answ.2, align 8

%tmp150 = fsub double 0.0, %tmp149
%tmp151 = load double, double* %tmp123, align 8
%tmp152 = fcmp oeq double %tmp150, %tmp151
br i1 %tmp152, label %then.11, label %else.11
then.11:
%tmp153 = load double, double* %answ.2, align 8

%tmp154 = fsub double 0.0, %tmp153
store double %tmp154, double* %tmp158, align 8
br label %fi.11
else.11:
%tmp155 = load double, double* %answ.2, align 8

%tmp156 = fsub double 0.0, %tmp155
%tmp157 = fsub double %tmp156, 1.0
store double %tmp157, double* %tmp158, align 8
br label %fi.11
fi.11:
%tmp159 = load double, double* %tmp158, align 8
store double %tmp159, double* %tmp160, align 8
br label %fi.12
fi.12:
%tmp161 = load double, double* %tmp160, align 8
  ret double %tmp161
}

define double @sin(double %x) {
entry:
%tmp162 = alloca double, align 8
store double %x, double* %tmp162, align 8

%running_factorial.1 = alloca double, align 8
store double 6.0, double* %running_factorial.1, align 8
%last_factor_of_factorial.1 = alloca double, align 8
store double 3.0, double* %last_factor_of_factorial.1, align 8
%PI.0 = alloca double, align 8
store double 3.141592653589793, double* %PI.0, align 8
%tmp163 = load double, double* %tmp162, align 8
%tmp164 = load double, double* %tmp162, align 8
%tmp165 = load double, double* %PI.0, align 8
%tmp166 = fdiv double %tmp164, %tmp165
%tmp167 = fdiv double %tmp166, 2.0
  %tmp168 = call double @floor(double %tmp167)
%tmp169 = load double, double* %PI.0, align 8
%tmp170 = fmul double %tmp168, %tmp169
%tmp171 = fmul double %tmp170, 2.0
%tmp172 = fsub double %tmp163, %tmp171
%x.1 = alloca double, align 8
store double %tmp172, double* %x.1, align 8
%tmp173 = load double, double* %x.1, align 8
%answ.3 = alloca double, align 8
store double %tmp173, double* %answ.3, align 8
%tmp174 = load double, double* %x.1, align 8
%last_term.1 = alloca double, align 8
store double %tmp174, double* %last_term.1, align 8
%add.0 = alloca i1, align 1
store i1 false, i1* %add.0, align 1

br label %loop.14
loop.14:
%tmp175 = load double, double* %last_term.1, align 8
%tmp176 = fcmp ogt double %tmp175, 0.0000000001
br i1 %tmp176, label %body.14, label %loop_exit.14
body.14:

%tmp177 = load double, double* %x.1, align 8
%tmp178 = load double, double* %last_factor_of_factorial.1, align 8
  %tmp179 = call double @integer_pow(double %tmp177, double %tmp178)
%tmp180 = load double, double* %running_factorial.1, align 8
%tmp181 = fdiv double %tmp179, %tmp180
store double %tmp181, double* %last_term.1, align 8

%tmp182 = load double, double* %answ.3, align 8
%tmp187 = alloca double, align 8
%tmp183 = load i1, i1* %add.0, align 1
br i1 %tmp183, label %then.13, label %else.13
then.13:
%tmp184 = load double, double* %last_term.1, align 8
store double %tmp184, double* %tmp187, align 8
br label %fi.13
else.13:
%tmp185 = load double, double* %last_term.1, align 8

%tmp186 = fsub double 0.0, %tmp185
store double %tmp186, double* %tmp187, align 8
br label %fi.13
fi.13:
%tmp188 = load double, double* %tmp187, align 8
%tmp189 = fadd double %tmp182, %tmp188
store double %tmp189, double* %answ.3, align 8

%tmp190 = load i1, i1* %add.0, align 1

%tmp191 = xor i1 %tmp190, true
store i1 %tmp191, i1* %add.0, align 1

%tmp192 = load double, double* %last_factor_of_factorial.1, align 8
%tmp193 = fadd double %tmp192, 1.0
store double %tmp193, double* %last_factor_of_factorial.1, align 8

%tmp194 = load double, double* %running_factorial.1, align 8
%tmp195 = load double, double* %last_factor_of_factorial.1, align 8
%tmp196 = fmul double %tmp194, %tmp195
store double %tmp196, double* %running_factorial.1, align 8

%tmp197 = load double, double* %last_factor_of_factorial.1, align 8
%tmp198 = fadd double %tmp197, 1.0
store double %tmp198, double* %last_factor_of_factorial.1, align 8

%tmp199 = load double, double* %running_factorial.1, align 8
%tmp200 = load double, double* %last_factor_of_factorial.1, align 8
%tmp201 = fmul double %tmp199, %tmp200
store double %tmp201, double* %running_factorial.1, align 8
br label %loop.14
loop_exit.14:

%tmp202 = load double, double* %answ.3, align 8
  ret double %tmp202
}

define double @cos(double %x) {
entry:
%tmp203 = alloca double, align 8
store double %x, double* %tmp203, align 8
%tmp204 = fdiv double 3.141592653589793, 2.0
%tmp205 = load double, double* %tmp203, align 8
%tmp206 = fadd double %tmp204, %tmp205
  %tmp207 = call double @sin(double %tmp206)
  ret double %tmp207
}

define double @tan(double %x) {
entry:
%tmp208 = alloca double, align 8
store double %x, double* %tmp208, align 8
%tmp209 = load double, double* %tmp208, align 8
  %tmp210 = call double @sin(double %tmp209)
%tmp211 = load double, double* %tmp208, align 8
  %tmp212 = call double @cos(double %tmp211)
%tmp213 = fdiv double %tmp210, %tmp212
  ret double %tmp213
}

define i32 @main() {
entry:
%PI.1 = alloca double, align 8
store double 3.141592653589793, double* %PI.1, align 8
%E.0 = alloca double, align 8
store double 2.718281828459045, double* %E.0, align 8

%tmp215 = call i8* @malloc(i64 4)
%tmp216 = getelementptr inbounds [4 x i8], [4 x i8]* @tmp214_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp215, i8* %tmp216)
%tmp217 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp217, i8* %tmp215)

  %tmp218 = call double @integer_pow(double 3.0, double 3.0)
%tmp219 = fsub double %tmp218, 27.0
%tmp220 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp220, double %tmp219)

  %tmp221 = call double @pow(double 3.0, double 3.0)
%tmp222 = fsub double %tmp221, 27.0
%tmp223 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp223, double %tmp222)

%tmp225 = call i8* @malloc(i64 1)
%tmp226 = getelementptr inbounds [1 x i8], [1 x i8]* @tmp224_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp225, i8* %tmp226)
%tmp227 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp227, i8* %tmp225)

%tmp229 = call i8* @malloc(i64 4)
%tmp230 = getelementptr inbounds [4 x i8], [4 x i8]* @tmp228_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp229, i8* %tmp230)
%tmp231 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp231, i8* %tmp229)

  %tmp232 = call double @exp(double 2.0)
%tmp233 = fsub double %tmp232, 7.3890560989306495
%tmp234 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp234, double %tmp233)

  %tmp235 = call double @exp(double 20.0)
%tmp236 = fsub double %tmp235, 485165195.40978974
%tmp237 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp237, double %tmp236)


%tmp238 = fsub double 0.0, 10.0
  %tmp239 = call double @exp(double %tmp238)
%tmp240 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp240, double %tmp239)

%tmp242 = call i8* @malloc(i64 1)
%tmp243 = getelementptr inbounds [1 x i8], [1 x i8]* @tmp241_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp242, i8* %tmp243)
%tmp244 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp244, i8* %tmp242)

%tmp246 = call i8* @malloc(i64 6)
%tmp247 = getelementptr inbounds [6 x i8], [6 x i8]* @tmp245_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp246, i8* %tmp247)
%tmp248 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp248, i8* %tmp246)

  %tmp249 = call double @floor(double 2.6)
%tmp250 = fsub double %tmp249, 2.0
%tmp251 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp251, double %tmp250)

  %tmp252 = call double @floor(double 3000.0)
%tmp253 = fsub double %tmp252, 3000.0
%tmp254 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp254, double %tmp253)

  %tmp255 = call double @floor(double 124.321)
%tmp256 = fsub double %tmp255, 124.0
%tmp257 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp257, double %tmp256)

  %tmp258 = call double @floor(double 12345.1212424)
%tmp259 = fsub double %tmp258, 12345.0
%tmp260 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp260, double %tmp259)

%tmp262 = call i8* @malloc(i64 1)
%tmp263 = getelementptr inbounds [1 x i8], [1 x i8]* @tmp261_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp262, i8* %tmp263)
%tmp264 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp264, i8* %tmp262)

%tmp266 = call i8* @malloc(i64 3)
%tmp267 = getelementptr inbounds [3 x i8], [3 x i8]* @tmp265_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp266, i8* %tmp267)
%tmp268 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp268, i8* %tmp266)

%tmp269 = load double, double* %E.0, align 8
  %tmp270 = call double @ln(double %tmp269)
%tmp271 = fsub double %tmp270, 1.0
%tmp272 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp272, double %tmp271)

%tmp273 = load double, double* %E.0, align 8
%tmp274 = load double, double* %E.0, align 8
%tmp275 = fmul double %tmp273, %tmp274
  %tmp276 = call double @ln(double %tmp275)
%tmp277 = fsub double %tmp276, 2.0
%tmp278 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp278, double %tmp277)

%tmp279 = load double, double* %E.0, align 8
%tmp280 = load double, double* %E.0, align 8
%tmp281 = fmul double %tmp279, %tmp280
%tmp282 = load double, double* %E.0, align 8
%tmp283 = fmul double %tmp281, %tmp282
%tmp284 = load double, double* %E.0, align 8
%tmp285 = fmul double %tmp283, %tmp284
  %tmp286 = call double @ln(double %tmp285)
%tmp287 = fsub double %tmp286, 4.0
%tmp288 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp288, double %tmp287)

  %tmp289 = call double @ln(double 20000000.0)
%tmp290 = fsub double %tmp289, 16.811242831518264
%tmp291 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp291, double %tmp290)

  %tmp292 = call double @ln(double 0.000002)

%tmp293 = fsub double 0.0, 13.122363377404328
%tmp294 = fsub double %tmp292, %tmp293
%tmp295 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp295, double %tmp294)

%tmp297 = call i8* @malloc(i64 1)
%tmp298 = getelementptr inbounds [1 x i8], [1 x i8]* @tmp296_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp297, i8* %tmp298)
%tmp299 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp299, i8* %tmp297)

%tmp301 = call i8* @malloc(i64 4)
%tmp302 = getelementptr inbounds [4 x i8], [4 x i8]* @tmp300_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp301, i8* %tmp302)
%tmp303 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp303, i8* %tmp301)

%tmp305 = call i8* @malloc(i64 4)
%tmp306 = getelementptr inbounds [4 x i8], [4 x i8]* @tmp304_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp305, i8* %tmp306)
%tmp307 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp307, i8* %tmp305)

%tmp308 = load double, double* %PI.1, align 8
%tmp309 = fdiv double %tmp308, 3.0
  %tmp310 = call double @sin(double %tmp309)
  %tmp311 = call double @sqrt(double 3.0)
%tmp312 = fdiv double %tmp311, 2.0
%tmp313 = fsub double %tmp310, %tmp312
%tmp314 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp314, double %tmp313)

%tmp315 = load double, double* %PI.1, align 8
%tmp316 = fdiv double %tmp315, 6.0
  %tmp317 = call double @sin(double %tmp316)
%tmp318 = fsub double %tmp317, 0.5
%tmp319 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp319, double %tmp318)

%tmp320 = load double, double* %PI.1, align 8
  %tmp321 = call double @sin(double %tmp320)
%tmp322 = fsub double %tmp321, 0.0
%tmp323 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp323, double %tmp322)

%tmp324 = load double, double* %PI.1, align 8
%tmp325 = fdiv double %tmp324, 2.0
  %tmp326 = call double @sin(double %tmp325)
%tmp327 = fsub double %tmp326, 1.0
%tmp328 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp328, double %tmp327)

  %tmp329 = call double @sin(double 300.0)

%tmp330 = fsub double 0.0, 0.9997558399011495
%tmp331 = fsub double %tmp329, %tmp330
%tmp332 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp332, double %tmp331)

%tmp334 = call i8* @malloc(i64 1)
%tmp335 = getelementptr inbounds [1 x i8], [1 x i8]* @tmp333_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp334, i8* %tmp335)
%tmp336 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp336, i8* %tmp334)

%tmp338 = call i8* @malloc(i64 4)
%tmp339 = getelementptr inbounds [4 x i8], [4 x i8]* @tmp337_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp338, i8* %tmp339)
%tmp340 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp340, i8* %tmp338)

%tmp341 = load double, double* %PI.1, align 8
  %tmp342 = call double @cos(double %tmp341)
%tmp343 = fadd double %tmp342, 1.0
%tmp344 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp344, double %tmp343)

%tmp345 = load double, double* %PI.1, align 8
%x.2 = alloca double, align 8
store double %tmp345, double* %x.2, align 8
%tmp346 = load double, double* %x.2, align 8
  %tmp347 = call double @sin(double %tmp346)
  %tmp348 = call double @integer_pow(double %tmp347, double 2.0)
%tmp349 = load double, double* %x.2, align 8
  %tmp350 = call double @cos(double %tmp349)
  %tmp351 = call double @integer_pow(double %tmp350, double 2.0)
%tmp352 = fadd double %tmp348, %tmp351
%tmp353 = fsub double %tmp352, 1.0
%tmp354 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp354, double %tmp353)

%tmp355 = load double, double* %PI.1, align 8
%tmp356 = fdiv double %tmp355, 4.0
%x.3 = alloca double, align 8
store double %tmp356, double* %x.3, align 8
%tmp357 = load double, double* %x.3, align 8
  %tmp358 = call double @sin(double %tmp357)
  %tmp359 = call double @integer_pow(double %tmp358, double 2.0)
%tmp360 = load double, double* %x.3, align 8
  %tmp361 = call double @cos(double %tmp360)
  %tmp362 = call double @integer_pow(double %tmp361, double 2.0)
%tmp363 = fadd double %tmp359, %tmp362
%tmp364 = fsub double %tmp363, 1.0
%tmp365 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp365, double %tmp364)

%x.4 = alloca double, align 8
store double 300.0, double* %x.4, align 8
%tmp366 = load double, double* %x.4, align 8
  %tmp367 = call double @sin(double %tmp366)
  %tmp368 = call double @integer_pow(double %tmp367, double 2.0)
%tmp369 = load double, double* %x.4, align 8
  %tmp370 = call double @cos(double %tmp369)
  %tmp371 = call double @integer_pow(double %tmp370, double 2.0)
%tmp372 = fadd double %tmp368, %tmp371
%tmp373 = fsub double %tmp372, 1.0
%tmp374 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp374, double %tmp373)

%tmp376 = call i8* @malloc(i64 1)
%tmp377 = getelementptr inbounds [1 x i8], [1 x i8]* @tmp375_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp376, i8* %tmp377)
%tmp378 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp378, i8* %tmp376)

%tmp380 = call i8* @malloc(i64 4)
%tmp381 = getelementptr inbounds [4 x i8], [4 x i8]* @tmp379_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp380, i8* %tmp381)
%tmp382 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp382, i8* %tmp380)

%tmp383 = load double, double* %PI.1, align 8
%tmp384 = fdiv double %tmp383, 4.0
  %tmp385 = call double @tan(double %tmp384)
%tmp386 = fsub double %tmp385, 1.0
%tmp387 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp387, double %tmp386)

%tmp389 = call i8* @malloc(i64 1)
%tmp390 = getelementptr inbounds [1 x i8], [1 x i8]* @tmp388_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp389, i8* %tmp390)
%tmp391 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp391, i8* %tmp389)

%tmp393 = call i8* @malloc(i64 5)
%tmp394 = getelementptr inbounds [5 x i8], [5 x i8]* @tmp392_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp393, i8* %tmp394)
%tmp395 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp395, i8* %tmp393)

  %tmp396 = call double @sqrt(double 4.0)
%tmp397 = fsub double %tmp396, 2.0
%tmp398 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp398, double %tmp397)

  %tmp399 = call double @sqrt(double 1024.0)
%tmp400 = fsub double %tmp399, 32.0
%tmp401 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp401, double %tmp400)

%tmp402 = load double, double* %PI.1, align 8
%tmp403 = fdiv double %tmp402, 3.0
  %tmp404 = call double @sin(double %tmp403)
  %tmp405 = call double @sqrt(double 3.0)
%tmp406 = fdiv double %tmp405, 2.0
%tmp407 = fsub double %tmp404, %tmp406
%tmp408 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp408, double %tmp407)

%tmp409 = load double, double* %PI.1, align 8
%tmp410 = fdiv double %tmp409, 4.0
  %tmp411 = call double @sin(double %tmp410)
  %tmp412 = call double @sqrt(double 2.0)
%tmp413 = fdiv double 1.0, %tmp412
%tmp414 = fsub double %tmp411, %tmp413
%tmp415 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp415, double %tmp414)

%tmp417 = call i8* @malloc(i64 1)
%tmp418 = getelementptr inbounds [1 x i8], [1 x i8]* @tmp416_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp417, i8* %tmp418)
%tmp419 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp419, i8* %tmp417)

ret i32 0
}
