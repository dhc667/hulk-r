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
@tmp4_str = private unnamed_addr constant [8 x i8] c"boniato\00", align 1

%A_vtable_type = type {
  i8*
}

@A_vtable = private unnamed_addr constant %A_vtable_type { i8* bitcast (i8* (%A_type*)* @A_get to i8*) }, align 8


%A_type = type { 
  %A_vtable_type*}


define %A_type* @A_new() {
entry:
  %tmp0 = call i8* @malloc(i64 8) ; Approx size
  %tmp1 = bitcast i8* %tmp0 to %A_type*
  %tmp2 = getelementptr inbounds %A_type, %A_type* %tmp1, i32 0, i32 0
  store %A_vtable_type* @A_vtable, %A_vtable_type** %tmp2, align 8
  ret %A_type* %tmp1
}

define i8* @A_get(%A_type* %self) {
entry:
  %tmp3 = alloca %A_type*, align 8
  store %A_type* %self, %A_type** %tmp3, align 8
%tmp5 = call i8* @malloc(i64 8)
%tmp6 = getelementptr inbounds [8 x i8], [8 x i8]* @tmp4_str, i64 0, i64 0
call i8* @strcpy(i8* %tmp5, i8* %tmp6)
  ret i8* %tmp5
}

define double @abs(double %x) {
entry:
%tmp7 = alloca double, align 8
store double %x, double* %tmp7, align 8
%tmp13 = alloca double, align 8
%tmp8 = load double, double* %tmp7, align 8
%tmp9 = fcmp ogt double %tmp8, 0.0
br i1 %tmp9, label %then.0, label %else.0
then.0:
%tmp10 = load double, double* %tmp7, align 8
store double %tmp10, double* %tmp13, align 8
br label %fi.0
else.0:
%tmp11 = load double, double* %tmp7, align 8

%tmp12 = fsub double 0.0, %tmp11
store double %tmp12, double* %tmp13, align 8
br label %fi.0
fi.0:
%tmp14 = load double, double* %tmp13, align 8
  ret double %tmp14
}

define double @integer_pow(double %base, double %exponent) {
entry:
%tmp15 = alloca double, align 8
store double %base, double* %tmp15, align 8
%tmp16 = alloca double, align 8
store double %exponent, double* %tmp16, align 8

%x.0 = alloca double, align 8
store double 0.0, double* %x.0, align 8
%answ.0 = alloca double, align 8
store double 1.0, double* %answ.0, align 8

br label %loop.1
loop.1:
%tmp17 = load double, double* %x.0, align 8
%tmp18 = load double, double* %tmp16, align 8
%tmp19 = fcmp olt double %tmp17, %tmp18
br i1 %tmp19, label %body.1, label %loop_exit.1
body.1:

%tmp20 = load double, double* %answ.0, align 8
%tmp21 = load double, double* %tmp15, align 8
%tmp22 = fmul double %tmp20, %tmp21
store double %tmp22, double* %answ.0, align 8

%tmp23 = load double, double* %x.0, align 8
%tmp24 = fadd double %tmp23, 1.0
store double %tmp24, double* %x.0, align 8
br label %loop.1
loop_exit.1:

%tmp25 = load double, double* %answ.0, align 8
  ret double %tmp25
}

define double @sqrt(double %x) {
entry:
%tmp26 = alloca double, align 8
store double %x, double* %tmp26, align 8

%tmp27 = load double, double* %tmp26, align 8
%xn.0 = alloca double, align 8
store double %tmp27, double* %xn.0, align 8
%tmp28 = load double, double* %tmp26, align 8
%tmp29 = fsub double %tmp28, 1.0
%xn_prev.0 = alloca double, align 8
store double %tmp29, double* %xn_prev.0, align 8

br label %loop.2
loop.2:
%tmp30 = load double, double* %xn.0, align 8
%tmp31 = load double, double* %xn_prev.0, align 8
%tmp32 = fsub double %tmp30, %tmp31
  %tmp33 = call double @abs(double %tmp32)
%tmp34 = fcmp olt double %tmp33, 0.00000001
br i1 %tmp34, label %body.2, label %loop_exit.2
body.2:

%tmp35 = load double, double* %xn.0, align 8
store double %tmp35, double* %xn_prev.0, align 8

%tmp36 = load double, double* %xn.0, align 8
%tmp37 = load double, double* %tmp26, align 8
%tmp38 = load double, double* %xn.0, align 8
%tmp39 = fdiv double %tmp37, %tmp38
%tmp40 = fadd double %tmp36, %tmp39
%tmp41 = fmul double 0.5, %tmp40
store double %tmp41, double* %xn.0, align 8
br label %loop.2
loop_exit.2:

%tmp42 = load double, double* %xn.0, align 8
  ret double %tmp42
}

define double @exp(double %x) {
entry:
%tmp43 = alloca double, align 8
store double %x, double* %tmp43, align 8

%running_factorial.0 = alloca double, align 8
store double 1.0, double* %running_factorial.0, align 8
%last_factor_of_factorial.0 = alloca double, align 8
store double 1.0, double* %last_factor_of_factorial.0, align 8
%answ.1 = alloca double, align 8
store double 1.0, double* %answ.1, align 8
%last_term.0 = alloca double, align 8
store double 1.0, double* %last_term.0, align 8
%tmp44 = load double, double* %tmp43, align 8
%tmp45 = fcmp olt double %tmp44, 0.0
%neg.0 = alloca i1, align 1
store i1 %tmp45, i1* %neg.0, align 1
%tmp46 = load double, double* %tmp43, align 8
  %tmp47 = call double @abs(double %tmp46)
%x.1 = alloca double, align 8
store double %tmp47, double* %x.1, align 8

br label %loop.3
loop.3:
%tmp48 = load double, double* %last_term.0, align 8
%tmp49 = fcmp ogt double %tmp48, 0.00000001
br i1 %tmp49, label %body.3, label %loop_exit.3
body.3:

%tmp50 = load double, double* %x.1, align 8
%tmp51 = load double, double* %last_factor_of_factorial.0, align 8
  %tmp52 = call double @integer_pow(double %tmp50, double %tmp51)
%tmp53 = load double, double* %running_factorial.0, align 8
%tmp54 = fdiv double %tmp52, %tmp53
store double %tmp54, double* %last_term.0, align 8

%tmp55 = load double, double* %answ.1, align 8
%tmp56 = load double, double* %last_term.0, align 8
%tmp57 = fadd double %tmp55, %tmp56
store double %tmp57, double* %answ.1, align 8

%tmp58 = load double, double* %last_factor_of_factorial.0, align 8
%tmp59 = fadd double %tmp58, 1.0
store double %tmp59, double* %last_factor_of_factorial.0, align 8

%tmp60 = load double, double* %running_factorial.0, align 8
%tmp61 = load double, double* %last_factor_of_factorial.0, align 8
%tmp62 = fmul double %tmp60, %tmp61
store double %tmp62, double* %running_factorial.0, align 8
br label %loop.3
loop_exit.3:

%tmp68 = alloca double, align 8
%tmp63 = load i1, i1* %neg.0, align 1

%tmp64 = xor i1 %tmp63, true
br i1 %tmp64, label %then.4, label %else.4
then.4:
%tmp65 = load double, double* %answ.1, align 8
store double %tmp65, double* %tmp68, align 8
br label %fi.4
else.4:
%tmp66 = load double, double* %answ.1, align 8
%tmp67 = fdiv double 1.0, %tmp66
store double %tmp67, double* %tmp68, align 8
br label %fi.4
fi.4:
%tmp69 = load double, double* %tmp68, align 8
  ret double %tmp69
}

define double @ln(double %x) {
entry:
%tmp70 = alloca double, align 8
store double %x, double* %tmp70, align 8

%tmp124 = alloca double, align 8
%tmp71 = load double, double* %tmp70, align 8
%tmp72 = fcmp ole double %tmp71, 0.0
br i1 %tmp72, label %then.8, label %else.8
then.8:

store double 0.0, double* %tmp124, align 8
br label %fi.8
else.8:

%int_part.0 = alloca double, align 8
store double 0.0, double* %int_part.0, align 8
%tmp73 = load double, double* %tmp70, align 8
%temp.0 = alloca double, align 8
store double %tmp73, double* %temp.0, align 8

br label %loop.5
loop.5:
%tmp74 = load double, double* %temp.0, align 8
%tmp75 = fcmp oge double %tmp74, 10.0
br i1 %tmp75, label %body.5, label %loop_exit.5
body.5:

%tmp76 = load double, double* %temp.0, align 8
%tmp77 = fdiv double %tmp76, 10.0
store double %tmp77, double* %temp.0, align 8

%tmp78 = load double, double* %int_part.0, align 8
%tmp79 = fadd double %tmp78, 1.0
store double %tmp79, double* %int_part.0, align 8
br label %loop.5
loop_exit.5:

br label %loop.6
loop.6:
%tmp80 = load double, double* %temp.0, align 8
%tmp81 = fcmp olt double %tmp80, 1.0
br i1 %tmp81, label %body.6, label %loop_exit.6
body.6:

%tmp82 = load double, double* %temp.0, align 8
%tmp83 = fmul double %tmp82, 10.0
store double %tmp83, double* %temp.0, align 8

%tmp84 = load double, double* %int_part.0, align 8
%tmp85 = fsub double %tmp84, 1.0
store double %tmp85, double* %int_part.0, align 8
br label %loop.6
loop_exit.6:

%tmp86 = load double, double* %temp.0, align 8
%tmp87 = fsub double %tmp86, 1.0
%tmp88 = load double, double* %temp.0, align 8
%tmp89 = fadd double %tmp88, 1.0
%tmp90 = fdiv double %tmp87, %tmp89
%y.0 = alloca double, align 8
store double %tmp90, double* %y.0, align 8
%tmp91 = load double, double* %y.0, align 8
%tmp92 = load double, double* %y.0, align 8
%tmp93 = fmul double %tmp91, %tmp92
%y2.0 = alloca double, align 8
store double %tmp93, double* %y2.0, align 8
%frac.0 = alloca double, align 8
store double 0.0, double* %frac.0, align 8
%tmp94 = load double, double* %y.0, align 8
%term.0 = alloca double, align 8
store double %tmp94, double* %term.0, align 8
%n.0 = alloca double, align 8
store double 0.0, double* %n.0, align 8
%epsilon.0 = alloca double, align 8
store double 0.00000001, double* %epsilon.0, align 8
%max_iter.0 = alloca double, align 8
store double 10000000.0, double* %max_iter.0, align 8

br label %loop.7
loop.7:
%tmp95 = load double, double* %term.0, align 8
  %tmp96 = call double @abs(double %tmp95)
%tmp97 = load double, double* %epsilon.0, align 8
%tmp98 = fcmp oge double %tmp96, %tmp97
%tmp99 = load double, double* %n.0, align 8
%tmp100 = load double, double* %max_iter.0, align 8
%tmp101 = fcmp olt double %tmp99, %tmp100
%tmp102 = and i1 %tmp98, %tmp101

br i1 %tmp102, label %body.7, label %loop_exit.7
body.7:

%tmp103 = load double, double* %frac.0, align 8
%tmp104 = load double, double* %term.0, align 8
%tmp105 = fadd double %tmp103, %tmp104
store double %tmp105, double* %frac.0, align 8

%tmp106 = load double, double* %n.0, align 8
%tmp107 = fadd double %tmp106, 1.0
store double %tmp107, double* %n.0, align 8

%tmp108 = load double, double* %term.0, align 8
%tmp109 = load double, double* %y2.0, align 8
%tmp110 = fmul double %tmp108, %tmp109
%tmp111 = load double, double* %n.0, align 8
%tmp112 = fmul double 2.0, %tmp111
%tmp113 = fsub double %tmp112, 1.0
%tmp114 = fmul double %tmp110, %tmp113
%tmp115 = load double, double* %n.0, align 8
%tmp116 = fmul double 2.0, %tmp115
%tmp117 = fadd double %tmp116, 1.0
%tmp118 = fdiv double %tmp114, %tmp117
store double %tmp118, double* %term.0, align 8
br label %loop.7
loop_exit.7:

%tmp119 = load double, double* %frac.0, align 8
%tmp120 = fmul double 2.0, %tmp119
%tmp121 = load double, double* %int_part.0, align 8
%tmp122 = fmul double %tmp121, 2.302585092994046
%tmp123 = fadd double %tmp120, %tmp122
store double %tmp123, double* %tmp124, align 8
br label %fi.8
fi.8:
%tmp125 = load double, double* %tmp124, align 8
  ret double %tmp125
}

define double @log(double %x, double %base) {
entry:
%tmp126 = alloca double, align 8
store double %x, double* %tmp126, align 8
%tmp127 = alloca double, align 8
store double %base, double* %tmp127, align 8
%tmp128 = load double, double* %tmp126, align 8
  %tmp129 = call double @ln(double %tmp128)
%tmp130 = load double, double* %tmp127, align 8
  %tmp131 = call double @ln(double %tmp130)
%tmp132 = fdiv double %tmp129, %tmp131
  ret double %tmp132
}

define double @pow(double %base, double %exponent) {
entry:
%tmp133 = alloca double, align 8
store double %base, double* %tmp133, align 8
%tmp134 = alloca double, align 8
store double %exponent, double* %tmp134, align 8
%tmp135 = load double, double* %tmp134, align 8
%tmp136 = load double, double* %tmp133, align 8
  %tmp137 = call double @ln(double %tmp136)
%tmp138 = fmul double %tmp135, %tmp137
  %tmp139 = call double @exp(double %tmp138)
  ret double %tmp139
}

define double @floor(double %x) {
entry:
%tmp140 = alloca double, align 8
store double %x, double* %tmp140, align 8

%tmp141 = load double, double* %tmp140, align 8
%tmp142 = fcmp olt double %tmp141, 0.0
%neg.1 = alloca i1, align 1
store i1 %tmp142, i1* %neg.1, align 1
%tmp143 = load double, double* %tmp140, align 8
%n.1 = alloca double, align 8
store double %tmp143, double* %n.1, align 8
%lower_bound.0 = alloca double, align 8
store double 1.0, double* %lower_bound.0, align 8
%answ.2 = alloca double, align 8
store double 0.0, double* %answ.2, align 8

br label %loop.9
loop.9:
%tmp144 = load double, double* %lower_bound.0, align 8
%tmp145 = fmul double %tmp144, 2.0
%tmp146 = load double, double* %n.1, align 8
%tmp147 = fcmp ole double %tmp145, %tmp146
br i1 %tmp147, label %body.9, label %loop_exit.9
body.9:

%tmp148 = load double, double* %lower_bound.0, align 8
%tmp149 = fmul double %tmp148, 2.0
store double %tmp149, double* %lower_bound.0, align 8
br label %loop.9
loop_exit.9:

br label %loop.11
loop.11:
%tmp150 = load double, double* %lower_bound.0, align 8
%tmp151 = fcmp oge double %tmp150, 1.0
br i1 %tmp151, label %body.11, label %loop_exit.11
body.11:

%tmp152 = load double, double* %n.1, align 8
%tmp153 = load double, double* %lower_bound.0, align 8
%tmp154 = fcmp oge double %tmp152, %tmp153
br i1 %tmp154, label %then.10, label %else.10
then.10:

%tmp155 = load double, double* %n.1, align 8
%tmp156 = load double, double* %lower_bound.0, align 8
%tmp157 = fsub double %tmp155, %tmp156
store double %tmp157, double* %n.1, align 8

%tmp158 = load double, double* %answ.2, align 8
%tmp159 = load double, double* %lower_bound.0, align 8
%tmp160 = fadd double %tmp158, %tmp159
store double %tmp160, double* %answ.2, align 8
br label %fi.10
else.10:

br label %fi.10
fi.10:

%tmp161 = load double, double* %lower_bound.0, align 8
%tmp162 = fdiv double %tmp161, 2.0
store double %tmp162, double* %lower_bound.0, align 8
br label %loop.11
loop_exit.11:

%tmp177 = alloca double, align 8
%tmp163 = load i1, i1* %neg.1, align 1

%tmp164 = xor i1 %tmp163, true
br i1 %tmp164, label %then.13, label %else.13
then.13:
%tmp165 = load double, double* %answ.2, align 8
store double %tmp165, double* %tmp177, align 8
br label %fi.13
else.13:

%tmp175 = alloca double, align 8
%tmp166 = load double, double* %answ.2, align 8

%tmp167 = fsub double 0.0, %tmp166
%tmp168 = load double, double* %tmp140, align 8
%tmp169 = fcmp oeq double %tmp167, %tmp168
br i1 %tmp169, label %then.12, label %else.12
then.12:
%tmp170 = load double, double* %answ.2, align 8

%tmp171 = fsub double 0.0, %tmp170
store double %tmp171, double* %tmp175, align 8
br label %fi.12
else.12:
%tmp172 = load double, double* %answ.2, align 8

%tmp173 = fsub double 0.0, %tmp172
%tmp174 = fsub double %tmp173, 1.0
store double %tmp174, double* %tmp175, align 8
br label %fi.12
fi.12:
%tmp176 = load double, double* %tmp175, align 8
store double %tmp176, double* %tmp177, align 8
br label %fi.13
fi.13:
%tmp178 = load double, double* %tmp177, align 8
  ret double %tmp178
}

define double @sin(double %x) {
entry:
%tmp179 = alloca double, align 8
store double %x, double* %tmp179, align 8

%running_factorial.1 = alloca double, align 8
store double 6.0, double* %running_factorial.1, align 8
%last_factor_of_factorial.1 = alloca double, align 8
store double 3.0, double* %last_factor_of_factorial.1, align 8
%PI.0 = alloca double, align 8
store double 3.141592653589793, double* %PI.0, align 8
%tmp180 = load double, double* %tmp179, align 8
%tmp181 = load double, double* %tmp179, align 8
%tmp182 = load double, double* %PI.0, align 8
%tmp183 = fdiv double %tmp181, %tmp182
%tmp184 = fdiv double %tmp183, 2.0
  %tmp185 = call double @floor(double %tmp184)
%tmp186 = load double, double* %PI.0, align 8
%tmp187 = fmul double %tmp185, %tmp186
%tmp188 = fmul double %tmp187, 2.0
%tmp189 = fsub double %tmp180, %tmp188
%x.2 = alloca double, align 8
store double %tmp189, double* %x.2, align 8
%tmp190 = load double, double* %x.2, align 8
%answ.3 = alloca double, align 8
store double %tmp190, double* %answ.3, align 8
%tmp191 = load double, double* %x.2, align 8
%last_term.1 = alloca double, align 8
store double %tmp191, double* %last_term.1, align 8
%add.0 = alloca i1, align 1
store i1 false, i1* %add.0, align 1

br label %loop.15
loop.15:
%tmp192 = load double, double* %last_term.1, align 8
%tmp193 = fcmp ogt double %tmp192, 0.0000000001
br i1 %tmp193, label %body.15, label %loop_exit.15
body.15:

%tmp194 = load double, double* %x.2, align 8
%tmp195 = load double, double* %last_factor_of_factorial.1, align 8
  %tmp196 = call double @integer_pow(double %tmp194, double %tmp195)
%tmp197 = load double, double* %running_factorial.1, align 8
%tmp198 = fdiv double %tmp196, %tmp197
store double %tmp198, double* %last_term.1, align 8

%tmp199 = load double, double* %answ.3, align 8
%tmp204 = alloca double, align 8
%tmp200 = load i1, i1* %add.0, align 1
br i1 %tmp200, label %then.14, label %else.14
then.14:
%tmp201 = load double, double* %last_term.1, align 8
store double %tmp201, double* %tmp204, align 8
br label %fi.14
else.14:
%tmp202 = load double, double* %last_term.1, align 8

%tmp203 = fsub double 0.0, %tmp202
store double %tmp203, double* %tmp204, align 8
br label %fi.14
fi.14:
%tmp205 = load double, double* %tmp204, align 8
%tmp206 = fadd double %tmp199, %tmp205
store double %tmp206, double* %answ.3, align 8

%tmp207 = load i1, i1* %add.0, align 1

%tmp208 = xor i1 %tmp207, true
store i1 %tmp208, i1* %add.0, align 1

%tmp209 = load double, double* %last_factor_of_factorial.1, align 8
%tmp210 = fadd double %tmp209, 1.0
store double %tmp210, double* %last_factor_of_factorial.1, align 8

%tmp211 = load double, double* %running_factorial.1, align 8
%tmp212 = load double, double* %last_factor_of_factorial.1, align 8
%tmp213 = fmul double %tmp211, %tmp212
store double %tmp213, double* %running_factorial.1, align 8

%tmp214 = load double, double* %last_factor_of_factorial.1, align 8
%tmp215 = fadd double %tmp214, 1.0
store double %tmp215, double* %last_factor_of_factorial.1, align 8

%tmp216 = load double, double* %running_factorial.1, align 8
%tmp217 = load double, double* %last_factor_of_factorial.1, align 8
%tmp218 = fmul double %tmp216, %tmp217
store double %tmp218, double* %running_factorial.1, align 8
br label %loop.15
loop_exit.15:

%tmp219 = load double, double* %answ.3, align 8
  ret double %tmp219
}

define double @cos(double %x) {
entry:
%tmp220 = alloca double, align 8
store double %x, double* %tmp220, align 8
%tmp221 = fdiv double 3.141592653589793, 2.0
%tmp222 = load double, double* %tmp220, align 8
%tmp223 = fadd double %tmp221, %tmp222
  %tmp224 = call double @sin(double %tmp223)
  ret double %tmp224
}

define double @tan(double %x) {
entry:
%tmp225 = alloca double, align 8
store double %x, double* %tmp225, align 8
%tmp226 = load double, double* %tmp225, align 8
  %tmp227 = call double @sin(double %tmp226)
%tmp228 = load double, double* %tmp225, align 8
  %tmp229 = call double @cos(double %tmp228)
%tmp230 = fdiv double %tmp227, %tmp229
  ret double %tmp230
}

define i32 @main() {
entry:
  %tmp231 = call %A_type* @A_new()
%a.0 = alloca i8*, align 8
store i8* %tmp231, i8** %a.0, align 8
%tmp232 = load i8*, i8** %a.0, align 8
  %tmp233 = bitcast i8* %tmp232 to %A_type*
  %tmp234 = getelementptr inbounds %A_type, %A_type* %tmp233, i32 0, i32 0
  %tmp235 = load %A_vtable_type*, %A_vtable_type** %tmp234, align 8
  %tmp236 = getelementptr inbounds %A_vtable_type, %A_vtable_type* %tmp235, i32 0, i32 0
  %tmp237 = load i8* (%A_type*)*, i8* (%A_type*)** %tmp236, align 8
  %tmp238 = call i8* %tmp237(%A_type* %tmp232)
%tmp239 = getelementptr inbounds [4 x i8], [4 x i8]* @.fmt, i64 0, i64 0
call i32 (i8*, ...) @printf(i8* %tmp239, i8* %tmp238)

ret i32 0
}
