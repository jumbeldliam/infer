mod common;

test_format!(Model, "model/gltf-binary", "glb", glb, "sample.glb");

test_format!(Model, "model/vnd.pixar.usd", "usd", usd, "sample.usd");

test_format!(Model, "model/fbx", "fbx", fbx, "sample.fbx");

test_format!(Model, "model/x-3ds", "3ds", threeds, "sample.3ds");

test_format!(Model, "model/sla", "stl", stl, "sample.stl");

test_format!(Model, "model/step", "step", step, "sample.step");

test_format!(Model, "model/drc", "drc", drc, "sample.drc");
