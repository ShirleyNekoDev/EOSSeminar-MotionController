void settings() {
  size(1000, 1000, P3D);
  smooth(2);
}

void setup() {
  frameRate(30);
}

float cameraRotationX = 0; // in rad
float cameraRotationY = -HALF_PI; // in rad
float cameraZoom = 0.5;

boolean renderAsWireframes = false;
void preDraw() {
  if(renderAsWireframes) {
    noFill();
    stroke(255);
    strokeWeight(1.5);
  } else {
    noStroke();
    fill(255);
    lights();
  }
}

void draw() {
  background(0);
  preDraw();
  
  pushMatrix();
  translate(width/2, height/2, 0);
  scale(cameraZoom);
  rotateX(cameraRotationX);
  rotateY(cameraRotationY);
  translate(0, 0, (upperArmLength + lowerArmLength)/2);
  
  // torso
  pushMatrix();
  translate(0, torsoHeight/2, 0);
  box(shoulderWidth, torsoHeight, torsoDepth);
  popMatrix();
  
  //head
  pushMatrix();
  sphereDetail(8);
  translate(0, -headHeight/2, 0);
  sphere(headHeight/2);
  popMatrix();
  
  // left arm
  pushMatrix();
  translate(-jointSize/2-shoulderWidth/2, jointSize/2, 0);
  drawArm(-1.0);
  popMatrix();
  
  // right arm
  pushMatrix();
  translate(jointSize/2+shoulderWidth/2, jointSize/2, 0);
  drawArm(1.0);
  popMatrix();
  popMatrix();
}

/**
 * position: 1.0 right, -1.0 left
*/
void drawArm(float position) {
  sphereDetail(4);
  
  // shoulder joint
  sphere(jointSize/2);
  
  // upper arm
  translate(0, 0, -upperArmLength/2);
  cylinder(armSize/2, upperArmLength);
  // upper arm sensor
  cylinder(sensorSize/4+armSize/2, sensorSize);
  pushMatrix();
  translate(position*armSize/2, 0, 0);
  box(sensorSize, sensorSize, sensorSize);
  popMatrix();
  
  // elbow joint
  translate(0, 0, -upperArmLength/2);
  sphere(jointSize/2);
  
  // lower arm
  translate(0, 0, -lowerArmLength/2);
  cylinder(armSize/2, lowerArmLength);
  // lower arm sensor
  cylinder(sensorSize/4+armSize/2, sensorSize);
  pushMatrix();
  translate(position*armSize/2, 0, 0);
  box(sensorSize, sensorSize, sensorSize);
  popMatrix();
  
  // wrist joint
  translate(0, 0, -lowerArmLength/2);
  sphere(jointSize/2);
  
  // hand
  translate(0, 0, -handSize/2);
  sphere(handSize/2);
  
  // controller
  translate(0, 0, -controllerLength/2);
  box(controllerSize, controllerSize, controllerLength);
}

int cylinderDetails = 7;
void cylinder(float r, float h) {
  float angle = 360f / cylinderDetails;
  
  beginShape();
  for (int i = 0; i < cylinderDetails; i++) {
    float x = cos(radians(i * angle)) * r;
    float y = sin(radians(i * angle)) * r;
    vertex(x, y, -h/2);
  }
  endShape(CLOSE);
  
  beginShape();
    for (int i = 0; i < cylinderDetails; i++) {
    float x = cos(radians(i * angle)) * r;
    float y = sin(radians(i * angle)) * r;
    vertex(x, y, h/2);
  }
  endShape(CLOSE);
  
  beginShape(TRIANGLE_STRIP);
    for (int i = 0; i < cylinderDetails + 1; i++) {
    float x = cos(radians(i * angle)) * r;
    float y = sin(radians(i * angle)) * r;
    vertex(x, y, h/2);
    vertex(x, y, -h/2);    
  }
  endShape(CLOSE);
}
