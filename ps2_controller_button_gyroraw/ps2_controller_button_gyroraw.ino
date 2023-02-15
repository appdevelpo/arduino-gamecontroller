#include <basicMPU6050.h> 
basicMPU6050<> imu;
#include <Wire.h>

const uint8_t digitalpin[] = {16,8,10,11,9,7,5,23,22,17,12,13,14,15};
const char* port[6] = {"A0","A1","A2","A3","A6","A7"};
void setup(void) {
  Serial1.begin(115200);
  while (!Serial1) {
    delay(10); // will pause Zero, Leonardo, etc until Serial1 console opens
  }

  // Try to initialize!
  imu.setup();
  for (int i=0; i < 14; i++) {
    pinMode(digitalpin[i], INPUT);
  }
  for(int k = 0;k < 6;k++){
    pinMode(port[k],INPUT);
  }
  Serial1.println("");
  delay(100);
}

void loop() {
  Serial1.print(analogRead(A2));
  Serial1.print(" ");
  Serial1.print(analogRead(A3));
  Serial1.print(" ");
  //left analog sticks 
  Serial1.print(analogRead(A0));
  Serial1.print(" ");
  Serial1.print(analogRead(A1));
  Serial1.print(" ");
 //right trigger 
  Serial1.print(analogRead(A6)/4);
  Serial1.print(" ");
  //left trigger
  Serial1.print(analogRead(A7)/4);
  Serial1.print(" ");
  for(int i = 0; i< 14;i++){ 
    Serial1.print(digitalRead(digitalpin[i]));
  }
  Serial1.println("");
  Serial1.print("@");
  Serial1.print( " " );
  Serial1.print( imu.rawAx()-966 );
  Serial1.print( " " );
  Serial1.print( imu.rawAy()-964 );
  Serial1.print( " " );
  Serial1.print( imu.rawAz()+956);

  Serial1.print( " " );
  Serial1.print( imu.rawGx()+477 );
  Serial1.print( " " );
  Serial1.print( imu.rawGy()+107 );
  Serial1.print( " " );
  Serial1.println( imu.rawGz()+27 );
  delay(10);
}