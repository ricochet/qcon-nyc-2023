#include <math.h>

double exports_component_example_control_volt_var_ctrl(double setp, double alpha, double gamma, double delta, double vavg, double kv, double cap, double dert, double qprev) {
 double f1 = vavg - setp*kv - delta*kv;
 double f2 = setp*kv - vavg - delta*kv;

 if (f1 < 0) { f1 = 0; }
 if (f2 < 0) { f2 = 0; }

 double val = qprev - gamma*(qprev +alpha*f1 - alpha*f2);
 double thresh = sqrt(cap*cap - dert*dert);

 if (val > thresh) { val = thresh; }

 return val;
}