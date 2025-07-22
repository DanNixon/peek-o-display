module TopSection(thickness, display_offset) {
  difference() {
    linear_extrude(thickness) {
      difference() {
        // Main body
        square([110, 58], center = true);

        // Display area cutout
        translate([5, 0] + display_offset) {
          square([59, 45], center = true);
        }
      }
    }

    linear_extrude(2) {
      translate([2.5, 0] + display_offset) {
        square([70.5, 51], center = true);
      }
    }
  }

  translate([0, 0, -14.6]) {
    linear_extrude(14.6) {
      PlaceMountingHoles() {
        difference() {
          circle(d = 6, $fn = 24);
          circle(d = 2.95, $fn = 5);
        }
      }
    }
  }
}


module PlaceMountingHoles() {
  dx = 93.98 / 2;
  dy = 44.45 / 2;

  for(x = [-dx, dx]) {
    for(y = [-dy, dy]) {
      translate([x, y]) {
        children();
      }
    }
  }
}

TopSection(thickness = 3, display_offset = [0, 0]);
