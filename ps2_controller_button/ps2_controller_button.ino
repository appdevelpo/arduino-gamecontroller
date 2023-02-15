

// const char* lis[] = { "L2", "L1" };
const char* port[6] = {"A0","A1","A2","A3","A6","A7"};
const uint8_t digitalpin[] = {16,8,10,11,9,7,5,23,22,17,12,13,14,15};
void setup() {
  Serial1.begin(115200);
  // Serial1.println(digitalpin[0]);
  delay(100);
  for(int i=0; i< 6;i++){
    pinMode(port[i],INPUT);
  }
  for (int i=0; i < 14; i++) {
    pinMode(digitalpin[i], INPUT);
  }
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
  Serial1.print((analogRead(A6)/4));
  Serial1.print(" ");
  //left trigger
  Serial1.print((analogRead(A7)/4));
  Serial1.print(" ");
  for(int i = 0; i< 14;i++){ 
    Serial1.print(digitalRead(digitalpin[i]));
  }
  Serial1.println("");

  delay(7);
}
