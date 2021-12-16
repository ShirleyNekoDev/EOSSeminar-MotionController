void mouseDragged() {
  if(mouseButton == RIGHT) {
    cameraRotationX += (pmouseY - mouseY) * cameraRotationSensitivity;
    cameraRotationY += (mouseX - pmouseX) * cameraRotationSensitivity;
  }
}
void mouseWheel(MouseEvent event) {
  cameraZoom += -event.getCount() * cameraZoomSensitivity;
  if(cameraZoom < 0.45)
    cameraZoom = 0.45;
  else if(cameraZoom > 1.0)
    cameraZoom = 1.0;
}

void keyPressed() {
  if(key == 'w') {
    renderAsWireframes = !renderAsWireframes;
  }
}
