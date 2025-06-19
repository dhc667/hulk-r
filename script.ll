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
%tmp37 = load double, double* %tmp36, align 8
%tmp38 = fcmp olt double %tmp37, 0.0
%neg.0 = alloca i1, align 1
store i1 %tmp38, i1* %neg.0, align 1
%tmp39 = load double, double* %tmp36, align 8
  %tmp40 = call double @abs(double %tmp39)
%x.1 = alloca double, align 8
store double %tmp40, double* %x.1, align 8

br label %loop.3
loop.3:
%tmp41 = load double, double* %last_term.0, align 8
%tmp42 = fcmp ogt double %tmp41, 0.00000001
br i1 %tmp42, label %body.3, label %loop_exit.3
body.3:

%tmp43 = load double, double* %x.1, align 8
%tmp44 = load double, double* %last_factor_of_factorial.0, align 8
  %tmp45 = call double @integer_pow(double %tmp43, double %tmp44)
%tmp46 = load double, double* %running_factorial.0, align 8
%tmp47 = fdiv double %tmp45, %tmp46
store double %tmp47, double* %last_term.0, align 8

%tmp48 = load double, double* %answ.1, align 8
%tmp49 = load double, double* %last_term.0, align 8
%tmp50 = fadd double %tmp48, %tmp49
store double %tmp50, double* %answ.1, align 8

%tmp51 = load double, double* %last_factor_of_factorial.0, align 8
%tmp52 = fadd double %tmp51, 1.0
store double %tmp52, double* %last_factor_of_factorial.0, align 8

%tmp53 = load double, double* %running_factorial.0, align 8
%tmp54 = load double, double* %last_factor_of_factorial.0, align 8
%tmp55 = fmul double %tmp53, %tmp54
store double %tmp55, double* %running_factorial.0, align 8
br label %loop.3
loop_exit.3:

%tmp61 = alloca double, align 8
%tmp56 = load i1, i1* %neg.0, align 1

%tmp57 = xor i1 %tmp56, true
br i1 %tmp57, label %then.4, label %else.4
then.4:
%tmp58 = load double, double* %answ.1, align 8
store double %tmp58, double* %tmp61, align 8
br label %fi.4
else.4:
%tmp59 = load double, double* %answ.1, align 8
%tmp60 = fdiv double 1.0, %tmp59
store double %tmp60, double* %tmp61, align 8
br label %fi.4
fi.4:
%tmp62 = load double, double* %tmp61, align 8
  ret double %tmp62
}

define double @ln(double %x) {
entry:
%tmp63 = alloca double, align 8
store double %x, double* %tmp63, align 8

%tmp117 = alloca double, align 8
%tmp64 = load double, double* %tmp63, align 8
%tmp65 = fcmp ole double %tmp64, 0.0
br i1 %tmp65, label %then.8, label %else.8
then.8:

store double 0.0, double* %tmp117, align 8
br label %fi.8
else.8:

%int_part.0 = alloca double, align 8
store double 0.0, double* %int_part.0, align 8
%tmp66 = load double, double* %tmp63, align 8
%temp.0 = alloca double, align 8
store double %tmp66, double* %temp.0, align 8

br label %loop.5
loop.5:
%tmp67 = load double, double* %temp.0, align 8
%tmp68 = fcmp oge double %tmp67, 10.0
br i1 %tmp68, label %body.5, label %loop_exit.5
body.5:

%tmp69 = load double, double* %temp.0, align 8
%tmp70 = fdiv double %tmp69, 10.0
store double %tmp70, double* %temp.0, align 8

%tmp71 = load double, double* %int_part.0, align 8
%tmp72 = fadd double %tmp71, 1.0
store double %tmp72, double* %int_part.0, align 8
br label %loop.5
loop_exit.5:

br label %loop.6
loop.6:
%tmp73 = load double, double* %temp.0, align 8
%tmp74 = fcmp olt double %tmp73, 1.0
br i1 %tmp74, label %body.6, label %loop_exit.6
body.6:

%tmp75 = load double, double* %temp.0, align 8
%tmp76 = fmul double %tmp75, 10.0
store double %tmp76, double* %temp.0, align 8

%tmp77 = load double, double* %int_part.0, align 8
%tmp78 = fsub double %tmp77, 1.0
store double %tmp78, double* %int_part.0, align 8
br label %loop.6
loop_exit.6:

%tmp79 = load double, double* %temp.0, align 8
%tmp80 = fsub double %tmp79, 1.0
%tmp81 = load double, double* %temp.0, align 8
%tmp82 = fadd double %tmp81, 1.0
%tmp83 = fdiv double %tmp80, %tmp82
%y.0 = alloca double, align 8
store double %tmp83, double* %y.0, align 8
%tmp84 = load double, double* %y.0, align 8
%tmp85 = load double, double* %y.0, align 8
%tmp86 = fmul double %tmp84, %tmp85
%y2.0 = alloca double, align 8
store double %tmp86, double* %y2.0, align 8
%frac.0 = alloca double, align 8
store double 0.0, double* %frac.0, align 8
%tmp87 = load double, double* %y.0, align 8
%term.0 = alloca double, align 8
store double %tmp87, double* %term.0, align 8
%n.0 = alloca double, align 8
store double 0.0, double* %n.0, align 8
%epsilon.0 = alloca double, align 8
store double 0.00000001, double* %epsilon.0, align 8
%max_iter.0 = alloca double, align 8
store double 10000000.0, double* %max_iter.0, align 8

br label %loop.7
loop.7:
%tmp88 = load double, double* %term.0, align 8
  %tmp89 = call double @abs(double %tmp88)
%tmp90 = load double, double* %epsilon.0, align 8
%tmp91 = fcmp oge double %tmp89, %tmp90
%tmp92 = load double, double* %n.0, align 8
%tmp93 = load double, double* %max_iter.0, align 8
%tmp94 = fcmp olt double %tmp92, %tmp93
%tmp95 = and i1 %tmp91, %tmp94

br i1 %tmp95, label %body.7, label %loop_exit.7
body.7:

%tmp96 = load double, double* %frac.0, align 8
%tmp97 = load double, double* %term.0, align 8
%tmp98 = fadd double %tmp96, %tmp97
store double %tmp98, double* %frac.0, align 8

%tmp99 = load double, double* %n.0, align 8
%tmp100 = fadd double %tmp99, 1.0
store double %tmp100, double* %n.0, align 8

%tmp101 = load double, double* %term.0, align 8
%tmp102 = load double, double* %y2.0, align 8
%tmp103 = fmul double %tmp101, %tmp102
%tmp104 = load double, double* %n.0, align 8
%tmp105 = fmul double 2.0, %tmp104
%tmp106 = fsub double %tmp105, 1.0
%tmp107 = fmul double %tmp103, %tmp106
%tmp108 = load double, double* %n.0, align 8
%tmp109 = fmul double 2.0, %tmp108
%tmp110 = fadd double %tmp109, 1.0
%tmp111 = fdiv double %tmp107, %tmp110
store double %tmp111, double* %term.0, align 8
br label %loop.7
loop_exit.7:

%tmp112 = load double, double* %frac.0, align 8
%tmp113 = fmul double 2.0, %tmp112
%tmp114 = load double, double* %int_part.0, align 8
%tmp115 = fmul double %tmp114, 2.302585092994046
%tmp116 = fadd double %tmp113, %tmp115
store double %tmp116, double* %tmp117, align 8
br label %fi.8
fi.8:
%tmp118 = load double, double* %tmp117, align 8
  ret double %tmp118
}

define double @log(double %x, double %base) {
entry:
%tmp119 = alloca double, align 8
store double %x, double* %tmp119, align 8
%tmp120 = alloca double, align 8
store double %base, double* %tmp120, align 8
%tmp121 = load double, double* %tmp119, align 8
  %tmp122 = call double @ln(double %tmp121)
%tmp123 = load double, double* %tmp120, align 8
  %tmp124 = call double @ln(double %tmp123)
%tmp125 = fdiv double %tmp122, %tmp124
  ret double %tmp125
}

define double @pow(double %base, double %exponent) {
entry:
%tmp126 = alloca double, align 8
store double %base, double* %tmp126, align 8
%tmp127 = alloca double, align 8
store double %exponent, double* %tmp127, align 8
%tmp128 = load double, double* %tmp127, align 8
%tmp129 = load double, double* %tmp126, align 8
  %tmp130 = call double @ln(double %tmp129)
%tmp131 = fmul double %tmp128, %tmp130
  %tmp132 = call double @exp(double %tmp131)
  ret double %tmp132
}

define double @floor(double %x) {
entry:
%tmp133 = alloca double, align 8
store double %x, double* %tmp133, align 8

%tmp134 = load double, double* %tmp133, align 8
%tmp135 = fcmp olt double %tmp134, 0.0
%neg.1 = alloca i1, align 1
store i1 %tmp135, i1* %neg.1, align 1
%tmp136 = load double, double* %tmp133, align 8
%n.1 = alloca double, align 8
store double %tmp136, double* %n.1, align 8
%lower_bound.0 = alloca double, align 8
store double 1.0, double* %lower_bound.0, align 8
%answ.2 = alloca double, align 8
store double 0.0, double* %answ.2, align 8

br label %loop.9
loop.9:
%tmp137 = load double, double* %lower_bound.0, align 8
%tmp138 = fmul double %tmp137, 2.0
%tmp139 = load double, double* %n.1, align 8
%tmp140 = fcmp ole double %tmp138, %tmp139
br i1 %tmp140, label %body.9, label %loop_exit.9
body.9:

%tmp141 = load double, double* %lower_bound.0, align 8
%tmp142 = fmul double %tmp141, 2.0
store double %tmp142, double* %lower_bound.0, align 8
br label %loop.9
loop_exit.9:

br label %loop.11
loop.11:
%tmp143 = load double, double* %lower_bound.0, align 8
%tmp144 = fcmp oge double %tmp143, 1.0
br i1 %tmp144, label %body.11, label %loop_exit.11
body.11:

%tmp145 = load double, double* %n.1, align 8
%tmp146 = load double, double* %lower_bound.0, align 8
%tmp147 = fcmp oge double %tmp145, %tmp146
br i1 %tmp147, label %then.10, label %else.10
then.10:

%tmp148 = load double, double* %n.1, align 8
%tmp149 = load double, double* %lower_bound.0, align 8
%tmp150 = fsub double %tmp148, %tmp149
store double %tmp150, double* %n.1, align 8

%tmp151 = load double, double* %answ.2, align 8
%tmp152 = load double, double* %lower_bound.0, align 8
%tmp153 = fadd double %tmp151, %tmp152
store double %tmp153, double* %answ.2, align 8
br label %fi.10
else.10:

br label %fi.10
fi.10:

%tmp154 = load double, double* %lower_bound.0, align 8
%tmp155 = fdiv double %tmp154, 2.0
store double %tmp155, double* %lower_bound.0, align 8
br label %loop.11
loop_exit.11:

%tmp170 = alloca double, align 8
%tmp156 = load i1, i1* %neg.1, align 1

%tmp157 = xor i1 %tmp156, true
br i1 %tmp157, label %then.13, label %else.13
then.13:
%tmp158 = load double, double* %answ.2, align 8
store double %tmp158, double* %tmp170, align 8
br label %fi.13
else.13:

%tmp168 = alloca double, align 8
%tmp159 = load double, double* %answ.2, align 8

%tmp160 = fsub double 0.0, %tmp159
%tmp161 = load double, double* %tmp133, align 8
%tmp162 = fcmp oeq double %tmp160, %tmp161
br i1 %tmp162, label %then.12, label %else.12
then.12:
%tmp163 = load double, double* %answ.2, align 8

%tmp164 = fsub double 0.0, %tmp163
store double %tmp164, double* %tmp168, align 8
br label %fi.12
else.12:
%tmp165 = load double, double* %answ.2, align 8

%tmp166 = fsub double 0.0, %tmp165
%tmp167 = fsub double %tmp166, 1.0
store double %tmp167, double* %tmp168, align 8
br label %fi.12
fi.12:
%tmp169 = load double, double* %tmp168, align 8
store double %tmp169, double* %tmp170, align 8
br label %fi.13
fi.13:
%tmp171 = load double, double* %tmp170, align 8
  ret double %tmp171
}

define double @sin(double %x) {
entry:
%tmp172 = alloca double, align 8
store double %x, double* %tmp172, align 8

%running_factorial.1 = alloca double, align 8
store double 6.0, double* %running_factorial.1, align 8
%last_factor_of_factorial.1 = alloca double, align 8
store double 3.0, double* %last_factor_of_factorial.1, align 8
%PI.0 = alloca double, align 8
store double 3.141592653589793, double* %PI.0, align 8
%tmp173 = load double, double* %tmp172, align 8
%tmp174 = load double, double* %tmp172, align 8
%tmp175 = load double, double* %PI.0, align 8
%tmp176 = fdiv double %tmp174, %tmp175
%tmp177 = fdiv double %tmp176, 2.0
  %tmp178 = call double @floor(double %tmp177)
%tmp179 = load double, double* %PI.0, align 8
%tmp180 = fmul double %tmp178, %tmp179
%tmp181 = fmul double %tmp180, 2.0
%tmp182 = fsub double %tmp173, %tmp181
%x.2 = alloca double, align 8
store double %tmp182, double* %x.2, align 8
%tmp183 = load double, double* %x.2, align 8
%answ.3 = alloca double, align 8
store double %tmp183, double* %answ.3, align 8
%tmp184 = load double, double* %x.2, align 8
%last_term.1 = alloca double, align 8
store double %tmp184, double* %last_term.1, align 8
%add.0 = alloca i1, align 1
store i1 false, i1* %add.0, align 1

br label %loop.15
loop.15:
%tmp185 = load double, double* %last_term.1, align 8
%tmp186 = fcmp ogt double %tmp185, 0.0000000001
br i1 %tmp186, label %body.15, label %loop_exit.15
body.15:

%tmp187 = load double, double* %x.2, align 8
%tmp188 = load double, double* %last_factor_of_factorial.1, align 8
  %tmp189 = call double @integer_pow(double %tmp187, double %tmp188)
%tmp190 = load double, double* %running_factorial.1, align 8
%tmp191 = fdiv double %tmp189, %tmp190
store double %tmp191, double* %last_term.1, align 8

%tmp192 = load double, double* %answ.3, align 8
%tmp197 = alloca double, align 8
%tmp193 = load i1, i1* %add.0, align 1
br i1 %tmp193, label %then.14, label %else.14
then.14:
%tmp194 = load double, double* %last_term.1, align 8
store double %tmp194, double* %tmp197, align 8
br label %fi.14
else.14:
%tmp195 = load double, double* %last_term.1, align 8

%tmp196 = fsub double 0.0, %tmp195
store double %tmp196, double* %tmp197, align 8
br label %fi.14
fi.14:
%tmp198 = load double, double* %tmp197, align 8
%tmp199 = fadd double %tmp192, %tmp198
store double %tmp199, double* %answ.3, align 8

%tmp200 = load i1, i1* %add.0, align 1

%tmp201 = xor i1 %tmp200, true
store i1 %tmp201, i1* %add.0, align 1

%tmp202 = load double, double* %last_factor_of_factorial.1, align 8
%tmp203 = fadd double %tmp202, 1.0
store double %tmp203, double* %last_factor_of_factorial.1, align 8

%tmp204 = load double, double* %running_factorial.1, align 8
%tmp205 = load double, double* %last_factor_of_factorial.1, align 8
%tmp206 = fmul double %tmp204, %tmp205
store double %tmp206, double* %running_factorial.1, align 8

%tmp207 = load double, double* %last_factor_of_factorial.1, align 8
%tmp208 = fadd double %tmp207, 1.0
store double %tmp208, double* %last_factor_of_factorial.1, align 8

%tmp209 = load double, double* %running_factorial.1, align 8
%tmp210 = load double, double* %last_factor_of_factorial.1, align 8
%tmp211 = fmul double %tmp209, %tmp210
store double %tmp211, double* %running_factorial.1, align 8
br label %loop.15
loop_exit.15:

%tmp212 = load double, double* %answ.3, align 8
  ret double %tmp212
}

define double @cos(double %x) {
entry:
%tmp213 = alloca double, align 8
store double %x, double* %tmp213, align 8
%tmp214 = fdiv double 3.141592653589793, 2.0
%tmp215 = load double, double* %tmp213, align 8
%tmp216 = fadd double %tmp214, %tmp215
  %tmp217 = call double @sin(double %tmp216)
  ret double %tmp217
}

define double @tan(double %x) {
entry:
%tmp218 = alloca double, align 8
store double %x, double* %tmp218, align 8
%tmp219 = load double, double* %tmp218, align 8
  %tmp220 = call double @sin(double %tmp219)
%tmp221 = load double, double* %tmp218, align 8
  %tmp222 = call double @cos(double %tmp221)
%tmp223 = fdiv double %tmp220, %tmp222
  ret double %tmp223
}

define double** @a() {
entry:
%list_ptr_224 = call i8* @malloc(i64 16)
%casted_list_ptr_224 = bitcast i8* %list_ptr_224 to double*
%length_val_224 = add i64 1, 0
%length_ptr_224 = getelementptr inbounds double, double* %casted_list_ptr_224, i64 0
%casted_length_ptr_224 = bitcast double* %length_ptr_224 to i64*
store i64 %length_val_224, i64* %casted_length_ptr_224
%elem_ptr_224_0 = getelementptr inbounds double, double* %casted_list_ptr_224, i64 1
store double 2.0, double* %elem_ptr_224_0
%list_ptr_225 = call i8* @malloc(i64 16)
%casted_list_ptr_225 = bitcast i8* %list_ptr_225 to double**
%length_val_225 = add i64 1, 0
%length_ptr_225 = getelementptr inbounds double*, double** %casted_list_ptr_225, i64 0
%casted_length_ptr_225 = bitcast double** %length_ptr_225 to i64*
store i64 %length_val_225, i64* %casted_length_ptr_225
%elem_ptr_225_0 = getelementptr inbounds double*, double** %casted_list_ptr_225, i64 1
store double* %casted_list_ptr_224, double** %elem_ptr_225_0
  ret double** %casted_list_ptr_225
}

define i32 @main() {
entry:
  %tmp226 = call double** @a()
%x.3 = alloca i8*, align 8
store i8* %tmp226, i8** %x.3, align 8
%tmp227 = load i8*, i8** %x.3, align 8
  %list_elem_ptr_228 = bitcast i8* %tmp227 to double**
  %casted_index_228 = fptosi double 0.0 to i64
  %adjusted_index_228 = add i64 %casted_index_228, 1
  %elem_ptr_228 = getelementptr inbounds double*, double** %list_elem_ptr_228, i64 %adjusted_index_228
  %loaded_elem_228 = load double*, double** %elem_ptr_228
  %list_elem_ptr_229 = bitcast i8* %loaded_elem_228 to double*
  %casted_index_229 = fptosi double 0.0 to i64
  %adjusted_index_229 = add i64 %casted_index_229, 1
  %elem_ptr_229 = getelementptr inbounds double, double* %list_elem_ptr_229, i64 %adjusted_index_229
  %loaded_elem_229 = load double, double* %elem_ptr_229
%tmp230 = getelementptr inbounds [4 x i8], [4 x i8]* @.fstr, i32 0, i32 0
call i32 (i8*, ...) @printf(i8* %tmp230, double %loaded_elem_229)

ret i32 0
}
