OPENQASM 2.0;
include "qelib1.inc";
qreg q[16];
creg c[16];
gate OTHER ALLQUBITS { 
    h q1;
    h q2;
    h q3;
    x ALLQUBITS;
}
h q[0];
cx q[0],q[1];
h q[0];
