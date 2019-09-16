initSidebarItems({"struct":[["DirectionalLight","directional light struct `glsl,ignore struct DirectionalLight {    vec3 color;    float intensity;    vec3 direction; }; `"],["Environment","Environment Uniform `glsl,ignore uniform Environment {    vec3 ambient_color;    vec3 camera_position;    int point_light_count;    int directional_light_count;    int spot_light_count; }; `"],["JointsOffset","Instance-rate joints offset `glsl,ignore  uint joints_offset; `"],["Material","Material Uniform `glsl,ignore uniform Material {    UvOffset uv_offset;    float alpha_cutoff; }; `"],["PointLight","point light struct `glsl,ignore struct PointLight {    vec3 position;    vec3 color;    float intensity; }; `"],["SkinnedVertexArgs","Skinned Instance-rate vertex arguments. `glsl,ignore  mat4 model;  vec4 tint;  uint joints_offset: `"],["SpotLight","spot light struct `glsl,ignore struct SpotLight {    vec3 position;    vec3 color;    vec3 direction;    float angle;    float intensity;    float range;    float smoothness; }; `"],["SpriteArgs","Sprite Vertex Data `glsl,ignore vec2 dir_x; vec2 dir_y; vec2 pos; vec2 u_offset; vec2 v_offset; float depth; vec4 tint; `"],["TextureOffset","TextureOffset `glsl,ignore struct UvOffset {    vec2 u_offset;    vec2 v_offset; }; `"],["Tint","Tint `glsl,ignore vec4 tint; `"],["VertexArgs","Instance-rate vertex arguments `glsl,ignore  mat4 model;  vec4 tint; `"],["ViewArgs","ViewArgs `glsl,ignore uniform ViewArgs {    uniform mat4 proj;    uniform mat4 view; }; `"]],"trait":[["IntoPod","Trait for auto conversion into standard GLSL POD types."]]});