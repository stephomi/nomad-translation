// comments with "ICON FIT" means < 10 characters
// arguments with $0 $1 etc

// general stuffs
yes "Yes"
ok "Ok"
cancel "Cancel"
delete "Delete"
X "X"
Y "Y"
Z "Z"
noSelectedMesh "No selected mesh."
advancedSettings "Advanced"

// pbr
baseColor "Color"
roughness "Roughness"
metalness "Metalness"

// ------------------------------------------------------
// about
about.minify "Minify UI"
about.minify.help "You can also tap your screen with 4 fingers, if your device supports it."
about.turntable "Turntable"
about.turntableSpeed "Turntable Speed"
about.credits "Credits"
about.creditsOpenSource "Open-Source"
about.creditsArts "Matcaps & Hdris"

// ------------------------------------------------------
// alert
alert.confirmDelete "Confirm deletion?"
alert.confirmDelete.yes "Yes, delete"
alert.hole.nothing "The object has no holes!"
alert.shape.notVisible "The current mesh is invisible!"
alert.trim.nothing "Nothing to trim."
alert.trim.full  "Abort trim: the mesh is fully trimmed."
alert.mask.noExtract "Nothing to extract!"
alert.mask.noSplit "Nothing to split!"
alert.view.disabled "Features disabled in View Mode:"
alert.separate.fail "Could not separate: the object has only one part!"
alert.voxelRemesh.success "Remeshed!"
alert.voxelRemesh.empty "Abort remesh: result mesh has no faces."
alert.voxelRemesh.invalidInput "Invalid input!"
alert.matrix.clone "The object will be duplicated"
alert.gizmo.usePivot "Use custom pivot."
alert.gizmo.useAuto "Use automatic pivot."
alert.gizmo.editPivot "Edit pivot mode."
alert.gizmo.editObject "Edit object mode."
alert.dynamic.enable "Active dynamic topology"
alert.dynamic.disable "Disable dynamic topology"
alert.colorPicker "Drag your finger on the mesh to pick a color."
alert.camera.resetView "Reset view"
alert.camera.snapView "Snap view"
alert.mask.show "Show mask"
alert.mask.hide "Hide mask"
alert.selection.lock "Lock selection"
alert.selection.unlock "Unlock selection"
alert.selection.isolate "Isolate selection"
alert.selection.showAll "Show all"
alert.quickSave "Saving..."
alert.multiresLost "Multiresolution will be lost, proceed?"
// autosave popup
alert.autoSave.auto "Autosave in... $0s"
// bottom warning
alert.warning.needLayer "The current tool requires an active layer."
alert.warning.multiresLost "Multiresolution will be lost."
alert.warning.paintingHidden "Painting hidden: show it again in Settings panel."
alert.warning.noPartialWireframe "Partial drawing is disabled when wireframe is displayed."
// bottom tip
alert.tip.polygonDot "Press the green dot to apply the polygon."
alert.tip.shapeOrthographic "Consider using orthographic camera if you want to avoid perspective frustum distortion when using screen projector."
// undo
alert.state.trial "Undo canceled : trial version"

// ------------------------------------------------------
// background
background "Background"
background.settings "Settings"
background.color "Color"
background.environment "Environment"
background.blur "Blur"
background.exposure "Exposure"

background.imageEnable "Reference image"
background.imageX "Position X"
background.imageY "Position Y"
background.imageRotation "Rotation"
background.imageScale "Scale"
background.imageOverlay "Overlay"
background.imageReset "Reset settings"

// ------------------------------------------------------
// camera
camera "Camera"
// saved views
camera.updateView "Update view point?"
camera.addView "Add View"
camera.focusOn "Focus on"
// projection
camera.projection "Projection"
camera.orthographic "Orthographic"
camera.perspective "Perspective"
camera.fov "Fov"
// orbit
camera.orbit "Orbit mode"
camera.orbit.help "Trackball gives more degree of freedom you can also roll the camera with 2 fingers."
camera.trackball "Trackball"
camera.turntable "Turntable"
// speed
camera.speed "Speed"
camera.rotation "Rotation"
camera.panning "Panning"
camera.zooming "Zooming"
// misc
camera.resetView "Reset view"
camera.snapView "Snap view"
// interaction
camera.pivot "Pivot"
camera.doubleTapMesh "Double tap on mesh"
camera.doubleTapBackground "Double tap on background"
camera.doubleTapPivot "Update on double tap"
camera.doubleTapPivot.help "Update the rotation pivot when double tapping."
camera.autoPivot "Update on panning/zooming"
camera.autoPivot.help "Update the pivot when you interact with the camera with 2 fingers."
camera.doubleTapFocus "Focus"
camera.doubleTapFocus.help "When double tapping on the mesh the camera will pan and focus on the picked point."
camera.doubleTapFocusSelection "Focus on selection"
camera.doubleTapFocusSelection.help "When double taping on background focus on the selected mesh instead of the whole scene."

// scene and layer lists
curve.preset "Preset"
curve.custom "Custom"

// scene and layer lists
expandList "UI: Expand list"
expandList.help "Just an UI option for easier list management."

// ------------------------------------------------------
// file
file.project.empty "You have no saved project yet!"
file.project.unsaved "Unsaved changes!"
file.project.loseUnsaved "You will lose unsaved changes!"
file.project.lastManualSave "Preview of the last manual save"
file.project.trialNoOpen "Trial version: You won't be able to re-open the current project!"
file.project.trialOnlyOpen "Trial version: you can only re-open your current project!"

// project
file.project "Project"
file.project.save "Save"
file.project.save.confirm "Save $0?"
file.project.saveAs "Save As..."
file.project.saveAs.confirm "Overwrite $0?"
file.project.open "Open..."
file.project.open.confirm "Open $0?"
file.project.add "Add..."
file.project.add.confirm "Add $0 to the scene?"
file.project.new "New"
file.project.new.confirm "Create new scene?"
file.project.delete "Delete..."
file.project.delete.confirm "Delete $0?"
file.project.delete.confirmActive "Delete $0?

This is the current active project!"
file.project.delete.confirmOk "Are you sure?"

// autosave
file.project.autoSave "Project Auto Save"
file.project.autoSave.confirm "Disable Auto Save?"
file.project.autoSave.help "Save your project in a separate file at regular interval.
The autoSave file can be found in:

$0"
file.project.autoSave.popup "Popup timeout"
file.project.autoSave.minutes "Timer popup"
file.project.autoSave.delete "Discard Auto Save"
file.project.autoSave.delete.confirm "Confirm?"

// import
file.import.title "Import"
file.import.title.help "Supported format:
- Wavefront (.obj)
- glTF 2.0 (.glb .gltf)
- STL (.stl)"
file.importOpen "Open..."
file.importOpen.confirm "Import new file?"
file.import.add "Add..."
file.import.add.confirm "Import new file?"

file.exportSelection "Export selection only"
file.exportSelection.help "Export the current selected mesh instead of the entire scene."
file.convertToQuad "Reconstruct quad"
file.convertToQuad.help "Reconstruct quads from triangles by pairing triangle (if they are adjacent in the files)."

// export
file.export.title "Export"
file.export.title.help "If possible favor glTF export as it supports more features than other formats."

// gltf
file.export.gltf "Export glTF 2.0"
file.export.gltfLayer "Export layers"
file.export.gltfLayer.help "Export layers as morphs. Officially supported by glTF so it should work on other softwares as well."
file.export.gltfColor "Export colors"
file.export.gltfColor.help "Export vertex colors. Officially supported by glTF so it should work on other softwares as well."
file.export.gltfNormal "Export normals"
file.export.gltfNormal.help "Check this option if you want to open the file in another softwares.

Nomad does not need them."
file.export.gltfExtraPaint "Export extra paint"
file.export.gltfExtraPaint.help "Export roughness metalness mask and layer painting. This will be ignored by other softwares."

// obj
file.export.obj "Export OBJ"
file.export.objWarning "Layers and extra painting (roughness metalness and mask) will be lost."
file.export.objColorAppend "Export colors"
file.export.objColorAppend.help "Append color information after vertices.

Some 3d softwares will be able to read it but not all of them."
file.export.objColorHexa "Hexa color"
file.export.objColorHexa.help "Write color as hexadecimal (zbrush method).

Some 3d softwares will be able to read it but not all of them."

// stl
file.export.stl "Export STL"
file.export.stlWarning "Layers and extra painting (roughness metalness and mask) will be lost."
file.export.stlColor "Export colors"
file.export.stlColor.help "Some 3d softwares will be able to read it but not all of them."
file.export.stlAscii "By default the format is binary.

You can choose to export to text format (ASCII) but the file will be bigger."

file.settings.title "Settings"
file.settings.title.help "Most of the app settings are saved here (Camera Interface etc).

Some resources are saved separately and automatically these includes:
- HDRs
- Matcaps
- Alphas
- Backgrounds
- Projects

At the moment brush settings cannot be saved but custom brush managament is planned."

// settings
file.settings.reset "Reset to default"
file.settings.reset.confirm "Reset all settings?

Projects alphas matcaps hdris and backgrounds are not impacted."

// materials
file.materials "Material library"
file.materials.reset "Reset to default"
file.materials.reset.confirm "Reset material library?"

// tools
file.tools "Tools presets"
file.tools.reset "Reset to default"
file.tools.reset.confirm "Reset material library?"

// render
file.render "Render"
file.render.showInterface "Show interface"
file.render.size "Size"
file.render.screenResolution "screen"
file.render.export "Export png"
file.render.4kWarn "4K export might use lot of memory make sure to save first!"
file.render.transparent "Transparent background"
file.render.transparent.help "This option can be useful if you want to insert the mesh in a 2d creation software.

Partial object transparency is not supported for now."

// ------------------------------------------------------
// history
history "History"
history.root "Root"
history.undoConfirm "Do you confirm undoing all these operations?"
history.undoWarning "If you make an edit afterwards you might lose lot of changes."
history.stack "Stack"
history.limitSize "History limit (Mb)"
history.limitSize.help "Maximum size (in Mb) of the history.

The history will be updated on the next recorded operation."
history.limitStack "Stack limit"
history.limitStack.help "Maximum number of operation the application can keep.

The history will be updated on the next recorded operation."
history.rangeProtect "Range protection"
history.rangeProtect.help "If you go far in the history, it will prompt a confirm dialog before undoing many operations."
history.gesture "Quick gesture"
history.gesture.help "2-finger tap to undo.

3-finger tap to redo."
history.restoreCamera "Restore camera"
history.restoreCamera.help "Enable this option to restore the saved camera viewpoint when you undo/redo an action."
// display undo/redo
history.state.undo "Undo: $0"
history.state.redo "Redo: $0"
history.state.symmetrySplit "Symmetry Split"
history.state.voxelRemesh "Voxel remesh"
history.state.surfaceRemesh "Surface remesh"
// state multires
history.state.multiresToDynamic "Multires to Dynamic"
history.state.multiresLevel "Resolution change"
history.state.multiresSubdivide "Subdivide"
history.state.multiresReverse "Reversion"
history.state.multiresDeleteLower "Delete lower"
history.state.multiresDeleteHigher "Delete higher"
// mesh
history.state.meshDynamicToStatic "Dynamic to Static"
history.state.meshStaticToDynamic "Static to Dynamic"
history.state.meshSymmetryUpdate "Symmetry update"
history.state.meshMatrixUpdate "Matrix update"
history.state.meshVisibility "Visibility"
history.state.meshMaterial "Materil change"
// state scene
history.state.sceneAddRemove "Scene"
history.state.sceneMeshOrder "Mesh order"
// state layer
history.state.layerOrder "Move layer order $0"
history.state.layerMergeRedo "Unmerge layer $0"
history.state.layerCreate "Create layer $0"
history.state.layerDelete "Delete layer $0";
history.state.layerMerge "Merge layer $0";
history.state.layerHide "Hide layer $0"
history.state.layerShow "Show layer $0"
history.state.layerSelect "Select layer $0"
history.state.layerUnselect "Unselect layer $0"
history.state.layerName "Layer $0 name";
history.state.layerFactor "Layer $0 factor";
history.state.layerFactorOffset "Layer $0 offset factor";
history.state.layerFactorColor "Layer $0 color factor";
history.state.layerFactorRoughness "Layer $0 roughness factor";
history.state.layerFactorMetalness "Layer $0 metalness factor";
// state light
history.state.lightVisible "light $0 visible"
history.state.lightIntensity "Light $0 intensity"
history.state.lightColor "Light $0 color"
history.state.lightPosition "Light $0 position"
history.state.lightShadow "Light $0 shadow"
history.state.lightBias "Light $0 shadow bias"
history.state.lightAttachment "Attachment light $0"
history.state.lightAdd "Add light $0"
history.state.lightDelete "Delete light $0"
history.state.lightCopy "Copy light $0"
history.state.lightMove "Move light $0"
history.state.lightType "Light $0 type"
history.state.lightSpotAngle "Light $0 spot angle"
history.state.lightSpotSoftness "Light $0 spot softness"
history.state.lightRadius "Light $0 spot radius"

// ------------------------------------------------------
// pressure menu
input.useGlobal "Use global settings"
input.useGlobal.help "By default, the tools share the same pressure settings.

Uncheck this option if you want specific pressure settings for this tool."

input.pressure "Pressure"
input.pressureTitle "Pressure ($0)"
input.pressure.noTool "This tool doesn't use pen pressure."
input.pressure.noGrab "Grab stroke type will ignore pressure settings."
input.pressure.radius "Radius"
input.pressure.intensity "Intensity" 
input.pressure.useRadius "Active"
input.pressure.useIntensity "Active" 
input.pressure.curveRadius "Radius"
input.pressure.curveIntensity "Intensity"

input.cameraInteraction "Camera:"
input.sculptInteraction "Sculpt:"
input.interaction.fingerAndStylus "Finger and Stylus"
input.interaction.finger "Finger"
input.interaction.stylus "Stylus"

input.fingerSmooth "Finger always smooths"
input.unknownPressure "Allow unrecognized pressure"
input.unknownPressure.help "Check this option if the pressure doesn't work with your pencil or if you need pressure finger." 
// pencil
input.pencilAction.none "None"
input.pencilAction.smooth "Smooth"
input.pencilAction.alt "Add/Sub"
input.pencilAction.android "Pencil button"
input.pencilAction.android.help "Experimental"
input.pencilAction.ios "Pencil double tap"
input.pencilAction.ios.help "Only active for Apple Pencil 2nd gen."
// size rejection
input.useSizeRejection "Use size rejection"
input.useSizeRejectionWarning "If you are locked, quit and restart Nomad.
This setting is never saved."
input.useSizeRejectionConfirm "Make sure to save your project first in case you can't interact with the UI anymore."
input.useSizeRejection.help "Reject input if the contact area size larger than this value.

It might not work on every device."
input.sizeRejection "Max size threshold"
// help
input.interaction.title "Interaction" 
input.interaction.title.help "These options are always global."

// ------------------------------------------------------
// interface
interface "Interface"

// bottom buttons
interface.bottomButtons "Add shortcuts (bottom)..."
interface.quickVoxelRemesh "Voxel remesh"
interface.quickWireframe "Wireframe"
interface.quickLockSelection "Lock selection"
interface.quickLockSelection.help "When enabled, you cannot change the selection by tapping on a mesh."
interface.quickCameraReset "Camera reset"
interface.quickCameraSnap "Camera snap"
interface.quickCameraSnapFlip "Flip on already snap"
interface.quickCameraSnapFlip.help "If the camera is already snapped, the shortcut will mirror the view"

// left
interface.leftButtons "Add shortcuts (left)..."
interface.quickSmooth "Smooth"
interface.quickMask "Mask"
interface.quickToggle "Sub/Smooth/Mask toggle"
interface.quickPaint "Material"
interface.quickAlpha "Alpha"
interface.maskGesture "Mask gesture"
interface.screenTooSmall "If the screen is too small, some buttons won't be displayed."
interface.maskGesture.help "Hold the mask shortcut and:

- drag on the background to clear mask
- tap on the background to invert mask
- tap on the masked area to sharpen
- tap on the unmasked area to sharpen"

// colors
interface.colors "Main colors"
interface.colorSelect "Color widget"
interface.colorBase "Color base"
interface.colorBaseTransparent "Color panel" 
interface.panelTransparent "Transparent panel"
interface.blurFactor "Blur strength"

// color preset
interface.colorsPresets "Color presets"
interface.presetBlurRed "Red"
interface.presetBlurBlue "Blue"
interface.presetBlurGreen "Green"
interface.presetBlurYellow "Yellow"
interface.presetBlackWhite "Black & White"
interface.presetWhiteBlack "White & Black"
interface.presetLividOrange "Livid & Orange"
interface.presetCardboard "Cardboard"
interface.presetDefault "Default"

// style
interface.style "Style"
interface.resetAll "Reset Interface"
interface.resetAll.confirm "Reset interface settings?"
interface.flipTop "flip top"
interface.flipBottom "flip bottom"
interface.flipMiddle "flip middle"
interface.autoClose "Close panel on interaction"
interface.autoClose.help "Close the top panels when you start interaction with the canvas."
interface.showTooltips "Show tooltips"
interface.showPin "Show pin menu icon"
interface.showPin.help "The screen width needs to be big enough to support menu pinning."
interface.showTooltips.help "This is a tooltip."
interface.materialPreview "Material Picker preview"
interface.toolboxHide "Auto-hide toolbox"
interface.toolboxHide.help "Enable this option if you want to hide the toolbox."
interface.toolboxMaxColumn "Max column toolbox"
interface.rounding "Rounding"
interface.inlined "Slider inlined"
interface.dampingSlider "Damping slider"
interface.dampingSlider.help "Get more precision when you move away from the slider."
interface.curveToolSymmetric "Symmetric tool curve widget"
interface.curveToolSymmetric.help "The widget can be found in the Tool panel under the falloff option."
interface.animated "Animated"
interface.scale "Overall scale"
interface.cursorStep "Vertical spacing"
interface.panelWidth "Panel width"
interface.fontScale "Font scale"

// toolbox
interface.toolsOrder "Tools order"
interface.openOrderTools "Edit"
interface.resetOrderTools "Reset"
interface.resetOrderTools.confirm "Reset order?"

// debug
interface.debug "Debugging"
interface.debug.warning "For debugging only!"

// ------------------------------------------------------
// layer sub menu
layer.action "Action"
layer.name "Name"
layer.delete "Delete"
layer.move "Move"
layer.duplicate "Duplicate"
layer.mergeDown "Merge down"
layer.factors "Channel factors"
layer.offsetFactor "Position"
layer.colorFactor "Color"

// ------------------------------------------------------
// layers menu
layers.addLayer "Add layer"
layers.addLayerTrial "Trial version is limited to 1 layer per mesh."
layers.title "Layers"
layers.title.help "Layers can record position offsets and painting, it can be useful for non-linear workflow.
For example by experimenting different facial expression without relying on the history stack to undo the changes.

For painting data, layers are sorted in a top-down fashion; so layers on top will mask the lower ones.

In order to resolve the layer opacity, all painting data (color, roughness, metalness) share the same mask.
You can reset part of this mask (and thus, the layer influence) by using the 'DelLayer' tool.";
layers.multipleObjectWarning "Multiple meshes are selected, please select only one mesh."
layers.primitive "Layers are unavailable for primitives."
layers.baseSelected "None"

// ------------------------------------------------------
// light sub menu
light "Light"
light.color "Color"
light.intensity "Intensity"
light.attachment "Attachment"
light.attachment.fixed "Fixed"
light.attachment.camera "Camera"
light.attachment.environment "Environment"
light.attachment.help "-- Fixed
Light orientation won't change.

-- Camera
Light orientation depends on the camera view."
light.type "Type"
light.type.directional "Directional"
light.type.spot "Spot"
light.spotAngle "Cone angle"
light.spotSoftness "Softness"
light.radius "Radius falloff"
light.shadowCast "Shadow"
light.shadowNormalBias "Normal bias"

// ------------------------------------------------------
// material
material "Material"
material.addNew "Add new"
material.matcapWarning "Roughness and metalness will not be visible with matcap shading."
material.opacity = "Opacity"

// ------------------------------------------------------
// menu name (visible on small screen menu are collapsed)
menu.files "Files"
menu.scene "Scene"
menu.topology "Topology"
menu.render "Render"
menu.postProcess "PostProcess"
menu.camera "Camera"
menu.background "Background"
menu.tool "Tool"
menu.stroke "Stroke"
menu.paint "Paint"
menu.symmetry "Symmetry"
menu.pressure "Pressure"
menu.layers "Layers"
menu.settings "Settings"
menu.interface "Interface"
menu.history "History"
menu.historySettings "Settings"
menu.about "About"

// ------------------------------------------------------
// mesh sub menu
mesh.action "Action"
mesh.closeHoles "Close holes"
mesh.separate "Separate"
mesh.triplanarWarning "Layers, painting and multiresolution will be lost."
mesh.triplanarResolution "Resolution"
mesh.triplanarCubic "Force cubic"
mesh.triplanarConvert "Convert"
mesh.name "Name"
mesh.type "Type"
mesh.typeStatic "Static"
mesh.typeMultiresolution "Multiresolution"
mesh.typeDynamic "Dynamic"

// ------------------------------------------------------
// painting
paint.useGlobal "Global material"
paint.useGlobal.help "If this option is enabled, the selected material will be the same as the other tools"
paint.usePainting "Enabled" 
paint.useColor "Base color" 
paint.useRoughness "Roughness" 
paint.useMetalness "Metalness"
paint.intensity "Paint intensity"
paint.paintAll "Paint all" 
paint.paintAll.help "Apply the current material to the mesh."
paint.paintAllForce "Force paint all"
paint.paintAllForce.help "Apply the current material to the mesh.

Masked area and disabled channels won't be painted."
paint.strokePaintingTitle "Painting ($0)"
paint.layerWarning "Channel masking will be ignored if you try to apply it on a layer."

// ------------------------------------------------------
// postprocess
postprocess.mainEnable "Post Process" 
// fxaa
postprocess.fxaaEnable "Antialiasing (FXAA)"
// ssr
postprocess.ssrEnable "Reflection (SSR)" 
postprocess.ssrFactor "Strength" 
postprocess.ssrDistanceFading "Distance fading" 
postprocess.ssrDistanceFading.help "Attenuate the effect according to how far the reflection is.
It can help in hiding artefacts that the SSR suffers from."
postprocess.ssrMatcapWarning "SSR is only effective in PBR shading mode."
// ssao
postprocess.ssaoEnable "Ambient Occlusion" 
postprocess.ssaoRadius "Size" 
postprocess.ssaoFactor "Strength" 
postprocess.ssaoBias "Curvature bias" 
postprocess.ssaoBias.help "How sensitive the effect is depending on the surface curvature."
// dof
postprocess.dofEnable "Depth Of Field"
postprocess.dofBlurFar "Far blur" 
postprocess.dofBlurNear "Near blur"
postprocess.dofFocusTip "Tap an object to change the focus point."
// bloom
postprocess.bloomEnable "Bloom" 
postprocess.bloomIntensity "Intensity" 
postprocess.bloomRadius "Radius" 
postprocess.bloomRadius.help "How widespread the bloom is."
postprocess.bloomThreshold "Threshold" 
postprocess.bloomThreshold.help "Luminosity threshold to decide if a pixel will emit bloom or not.
If the value is at 0, everything will receive bloom."
// tone mapping
postprocess.toneEnable "Tone Mapping" 
postprocess.toneExposure "Exposure" 
postprocess.toneContrast "Contrast" 
postprocess.toneSaturation "Saturation" 
postprocess.toneMappingNone "None"
// chromatic
postprocess.chromaticEnable "Chromatic Aberration" 
postprocess.chromaticFactor "Strength" 
// vignette
postprocess.vignetteEnable "Vignette" 
postprocess.vignetteSize "Size" 
postprocess.vignetteHardness "Hardness" 
// sharpness
postprocess.sharpnessEnable "Sharpness" 
postprocess.sharpnessFactor "Strength" 
// grain
postprocess.grainEnable "Grain" 
postprocess.grainFactor "Strength" 

// ------------------------------------------------------
// primitive (scene menu)
primitive "Primitive"
primitive.box "Box"
primitive.sphere "Sphere"
primitive.sphereUV "UV Sphere"
primitive.icosahedron "Icosahedron"
primitive.cylinder "Cylinder"
primitive.cone "Cone"
primitive.torus "Torus"
primitive.plane "Plane"
primitive.triplanar "Triplanar"
primitive.needValidate "Primitives should be validated in order to be sculpted."

primitive.mainConfig "Parameter"
primitive.topology "Topology"
primitive.geometry "Geometry"

// common config
primitive.mirror "Mirroring"
primitive.mirror.help "Duplicate the primitiveb by using the symmetrical plane."
primitive.validate "Validate"
primitive.maxFaces "Max faces"
primitive.maxFaces.help "The maximum number of faces a primitive can have.

This limit is only active while the primitive is not validated, afterwards the safeguard is gone."
primitive.linear "Flat subdivision"
primitive.subdivision "Post subdivision"

// common config
primitive.size "Size"
primitive.sizeX "Size X"
primitive.sizeY "Size Y"
primitive.sizeZ "Size Z"
primitive.division "Division"
primitive.divisionX "Division X"
primitive.divisionY "Division Y"
primitive.divisionZ "Division Z"
primitive.angleX "Angle X"
primitive.angleY "Angle Y"
primitive.angleZ "Angle Z"
primitive.constantDensity "Constant density"
primitive.projectOnSphere "Project on sphere"
primitive.projectOnSphere.help "Snaps the points on a perfect sphere."

// triplanar
primitive.triplanar.title "Triplanar - Setting"
primitive.triplanar.title.help "Triplanar is using the mask information from 3 planes to fill a voxel grid that is then polygonized.

If you interact with the division or size sliders, the painting information will reset (smoothness is ok).

You should probably disable symmetry as it might not function as you would expect.

You can use the 'Topologically connected' option in the mask panel to paint a plane impacting the other planes."
primitive.triplanarIsolate "Visibility"
primitive.triplanarSameSize "Same size (cube)"
primitive.triplanarPolish "Smoothness"
primitive.triplanarResetMask "Reset mask"
primitive.triplanarReproject "Resize mask"
primitive.triplanarReproject.title "Reproject the plane mask when updating the triplanar settings.
\
If you uncheck this option, it will revert to the default spherical masks."
primitive.isolate.all "All"
primitive.isolate.back "Back"
primitive.isolate.right "Right"
primitive.isolate.bottom "Bottom"
// plane
primitive.planeSameSize "Same size (square)"
// box
primitive.boxRegular "Same size (cube)"
// icosahedron
primitive.icosahedronRadius "Radius"
// torus
primitive.torusRadiusOuter "Radius outer"
primitive.torusRadiusInner "Radius inner"
primitive.torusAngle "Angle"
primitive.torusHole "Hole"
// sphere
primitive.sphereRadius "Radius"
// cylinder
primitive.cylinderSameSize "Sync bottom/top radius"
primitive.cylinderRadiusBottom "Radius bottom"
primitive.cylinderRadiusTop "Radius top"
primitive.cylinderHeight "Height"
primitive.cylinderHole "Has hole"
// cone
primitive.coneRadius "Radius"
primitive.coneHeight "Height"

// common resources stuffs
resource.delete "Delete"
resource.import "Import"

// ------------------------------------------------------
// scene
scene.title "Scene"
scene.title.help "When using the selection checkbox, hold and drag your finger to select other objects easily."
scene.mergeSimple "Simple merge"
scene.mergeVoxel "Voxel merge"
scene.voxelResolution "Resolution"
scene.subtractionTip "Subtraction  : Hide mesh (eye icon)"
scene.intersectionTip "Intersection : All meshes hidden"

// ---------------------------
// settings
settings.displayTitle "Display settings"
settings.fingerRotateLighting "Rotate lighting (3 fingers)"
settings.fingerRotateLighting.help "Drag 3 fingers horizontally on the canvas to rotate the environment, lights and matcap."
// wireframe
settings.wireframeTitle "Wireframe"
settings.wireframeDisplay "Wireframe"
settings.wireframeColor "Wireframe color"
// backface
settings.backfaceTitle "Backface"
settings.backfaceVisible "Show backface"
settings.backfaceVisible.help "Backface faces are faces that point 'away' from the camera viewpoint.

All faces (triangle or quad) point to a certain direction, for example on a base sphere will see its faces point towards the outside.

If you move the camera inside the sphere you'll the backface of theses faces."
settings.backfaceColor "Backface color"
settings.backfaceColored "Colored backface"
// outline
settings.outlineTitle "Outline"
settings.outlineEnable "Outline"
settings.outlineThickness "Thickness"
settings.outlineColor "Color"
// grid
settings.gridTitle "Grid"
settings.gridDisplay "Grid"
settings.gridHeight "Height"
settings.gridColor "Color"
// cursor
settings.cursorWhileSculpting "Show circle while sculpting"
settings.cursorShowDot "Show small dot"
settings.cursorShowDot.help "The dot can appear as the camera pivot point or when you are sculpting."
settings.cursorShowRope "Show rope stabilizer"
// other
settings.renderRatio "Render resolution"
settings.darkenUnselected "Darken unselected meshes"
settings.smoothShading "Smooth shading"
settings.partialDraw "Partial drawing"
settings.partialDraw.help "Feature not polished!

Use it if you are sculpting a relatively small part of a high poly mesh.

It should make the sculpting smoother, but you should not enable wireframe!

Also it might add visual artefacts during the brush strokes"
settings.partialDrawWarning "Do not forget to turn off this option if the visual artefacts are too bothersome!"
settings.detailRangeTitle "Voxel/Dynamic remesh"
settings.detailRange "Max detail range"
settings.detailRange.help "Maximum value for voxel and dynamic topology level of detail.

Higher values mean more polygons, use at your own risk!"
settings.showPainting "Show scene painting"
settings.lightIcon "Light icons"
settings.lightIcon.help "Display light icons on the canvas so that you can select and edit them directly."
settings.showSnapCube "Snap cube"
settings.loadGuiSettings "Load project gui settings"
settings.loadGuiSettings.help "When opening or importing a project file, all the gui-related settings embedded in the project will be loaded."
settings.holeTitle "Hole-filling"
settings.holeNonManifold "Fill non-manifold"
settings.holeNonManifold.help "Try to fill non manifold hole.
This option is not saved in the settings.
"
settings.keepImportTopology "Keep topology at import"
settings.keepImportTopology.help "Use this option if you don't want Nomad to fiddle with the topology of imported mesh.

It will disable vertex/face reordering, removal of vertex/face duplicates and removal of unused vertices."
// multires
settings.multiresTitle "Multiresolution"
settings.multiresMaxVertices "Max vertices count"
settings.multiresMaxVertices.help "Nomad does not perform memory check before subdivision, high poly count can easily lead to crashes."
settings.multiresLowResVertices "Low resolution vertices threshold"
settings.multiresLowResVertices.help "A lower resolution of the mesh can be displayed when you move the camera.

You can increase this value if you want to display a higher resolution of the mesh."
// stat
settings.showSceneStats "Stats"
settings.statNone "None"
settings.statSelection "Selection"
settings.statAll "All"
// experimental
settings.experimentalTitle "Experimental"
settings.notSaved "These options are not saved in the settings."
// settings.parallel "Parallel sculpting"

// ------------------------------------------------------
// shading
shading "Shading"
// lights
shading.lights "Lights"
shading.lights.addLight "Add light"
shading.lights.matcapWarning "Lights are ignored in Matcap mode."
// environment
shading.environment "Environment"
shading.environmentImport "Import HDR"
shading.environmentExposure "Exposure"
shading.environmentRotation "Rotation"
shading.environmentRotation.help "You can rotate the environment by dragging 3 fingers horizontally on the viewport."
shading.environmentAttachedToCamera "Attached to camera"
shading.environmentAttachedToCamera.help "Attach the environment to the camera.

It will force the lighting to be consistent, which can be useful for sculpting purposes."
// matcap
shading.matcap "Matcap"
shading.matcapRotation "Rotation"
shading.matcapRotation.help "You can rotate the matcap by dragging 3 fingers horizontally on the viewport."
shading.matcapGlobal "Use global matcap"
shading.matcapGlobal.help "Uncheck this option to use a different matcap for this particular mesh."

// ------------------------------------------------------
// bottom shortcut buttons (ICON FIT)
shortcut.voxel "Voxel"
shortcut.wire "Wire"
shortcut.mask "Mask"
shortcut.reset "Reset"
shortcut.snap "Snap"
shortcut.solo "Solo"
shortcut.lock "Lock"

// ------------------------------------------------------
// stat
stat.ramScene "Scene"
stat.vramScene "Vram Scene"
stat.ramHistory "History"
stat.vramRender "Vram Render"
stat.ramOther "Other"
stat.usedMemory "Used Memory"
stat.freeMemory "Free Memory"
stat.total "Total"
stat.used "Used"
stat.free "Free"
stat.faces "Faces"
stat.triangles "Triangles"
stat.vertices "Vertices"
stat.quads "Quads"
stat.sceneFaces "Scene faces"
stat.sceneVertices "Scene vertices"

// ------------------------------------------------------
// stroke
stroke "Stroke"
strokeTitle "Stroke ($0)"
stroke.useWorldRadius "World-space radius"
stroke.useWorldRadius.help "This is shared among every tools."
stroke.useShareRadius "Share radius"
stroke.useShareRadius.help "Share the radius value among every tools."
stroke.minSpacingAdjustIntensity "Adjust spacing intensity"
stroke.minSpacingAdjustIntensity.help "Adjust the brush intensity to ensure consistent deformation depending on the stroke spacing."
stroke.minSpacing "Stroke spacing"
stroke.minSpacing.help "Spacing between each stroke, relative to the tool radius.

Lower value will allow smoother stroke but performance will degrade."
stroke.lazySmooth "Stroke smoothing"
stroke.lazySmooth.help "Average multiple pointer position to get a smoother stroke.

With high values, the stroke will lag behind the pointer but will eventually catch up."
stroke.lazyRadius "Lazy rope stabilizer"
stroke.lazyRadius.help "Strokes will lag behind the pointer position according to a certain distance.

This can be used to draw smooth lines."
stroke.globalSettings "This is a global setting"
stroke.snapRadius "Snap radius"
stroke.snapRadius.help "Snap the stroke if the pointer lies close to the last recorded stroke.

This can be useful when drawing long continuous lines, while doing frequent pauses."
stroke.sculptOffset "Stroke offset"
stroke.sculptOffset.help "Apply a constant offset on the stroke.

This option is there to help for small screen when using fingers, so that your finger doesn't cover the stroke."
stroke.accumulate "Accumulate stroke"
stroke.accumulate.help "If this option is enabled, there is no limit to how much matter you can add/remove per stroke."
stroke.useDynamicTopology "Allow dynamic topology"
stroke.connectedTopology "Connected topology"
stroke.connectedTopology.help "This option will only sculpt the vertices that are linked to the picked surface.

This is typically used for the Move tool, for example if you want to exclusively move a part that self-intersect with another part."
stroke.onlyFrontFace "Front-facing vertex only"
stroke.onlyFrontFace.help "This option will ignore back facing vertices.

It can be useful if you want to paint part of a thin geometry without impacting the other side.

It also works for sculpting but you might experience some artefacts."
stroke.intensityMultiplier "Intensity multiplier"
stroke.curveFalloff "Falloff"
stroke.onlyLasso "Settings only active for the lasso tool."
// alpha
stroke.alpha "Alpha" 
stroke.alphaInvert "Invert value"
stroke.alphaScale "Alpha scale"
stroke.alphaScale.help "At minimum value, the alpha square is inside the tool circle radius.

At maximum value, the tool circle radius is inside the alpha square."
// stroke type
stroke.strokeType "Stroke type"
stroke.strokeTypeDot "Dot"
stroke.strokeTypeDrag "Drag"
stroke.strokeTypeGrab "Grab"
stroke.strokeTypeGrabRadius "Grab - dynamic radius"
stroke.strokeTypeGrabIntensity "Grab - dynamic intensity"

// ------------------------------------------------------
// symmetry
symmetry "Symmetry"
symmetry.enable "Enabled"
symmetry.toolIgnore "The current tool ignores symmetry."
// method
symmetry.method "Method:"
symmetry.method.help "-- Local
The symmetry plane will move along the mesh when you use one of the transform tools (rotate, translate or gizmo).


-- World
The symmetry plane is fixed and will not move."
symmetry.methodWorld "World"
symmetry.methodLocal "Local"
// mirror
symmetry.mirror "Mirroring"
symmetry.mirror.help "Try to re-apply the symmetry without impacting the topology.

To succeed, the topology need to be symmetrical and at least one edge should lie exactly on the symmetry line.

If it fails, you will be proposed to force the symmetry, but it will impact the topology."
symmetry.apply "Mirror"
symmetry.flip "Flip direction"
symmetry.flip.help "Use this option to change the side in which the details are projected."
symmetry.applyFail "Failed to apply symmetry:
- $0

Do you want to enforce symmetry by mirroring the mesh?";
// reset
symmetry.resetOrigin "Reset Origin"
symmetry.resetCenterMesh "Mesh center"
symmetry.resetCenterWorld "World center"
symmetry.resetDirection "Reset Direction"
// advanced
symmetry.showLine "Show line"
symmetry.showPlane "Show plane"
symmetry.editWarning "Symmetry edit is experimental."
symmetry.edit "Gizmo edit"
symmetry.edit.help "You can freely set the symmetry plane.

This feature is a bit experimental and you should probably never use it."

// ------------------------------------------------------
// tools
// left bar generic (ICON FIT)
tool.sliderDegree "Rotate $0 °"
tool.sliderRadius "Radius $0 %"
tool.sliderIntensity "Intensity $0 %"
tool.dynTopo "DynTopo"
tool.symmetry "Sym"
tool.mirror "Mirror"
tool.clay "Clay"
tool.clay.sub "Sub"
tool.brush "Brush"
tool.move "Move"
tool.move.normal "Normal"
tool.drag "Drag"
tool.smooth "Smooth"
tool.smooth.relax "Relax"
tool.mask "Mask"
tool.mask.unmask "Unmask"
tool.maskSelector "SelMask"
tool.paint "Paint"
tool.paint.erase "Erase"
tool.smudge "Smudge"
tool.flatten "Flatten"
tool.flatten.fill "Fill"
tool.layer "Layer"
tool.crease "Crease"
tool.trim "Trim"
tool.split "Split"
tool.project "Project"
tool.inflate "Inflate"
tool.pinch "Pinch"
tool.nudge "Nudge"
tool.stamp "Stamp"
tool.clearLayer "DelLayer"
tool.gizmo "Gizmo"
tool.gizmo.auto "Auto"
tool.gizmo.editPivot "Pivot"
tool.gizmo.local "Local"
tool.transform "Transform"
tool.transform.move "Move"
tool.transform.rotate "Rotate"
tool.transform.scale "Scale"
tool.measure "Measure"
tool.view "View"
tool.shape.flip "Flip"
tool.shape.view "View"
tool.shape.lasso "Lasso"
tool.shape.curve "Curve"
tool.shape.polygon "Polygon"
tool.shape.rectangle "Rect"
tool.shape.ellipse "Ellipse"
tool.shape.line "Line"
// title
tool.settingsTitle "Settings ($0)"
// clay
tool.clay.flattenOffset "Flatten offset"
// crease
tool.crease.pinchFactor "Pinch force"
// layer
tool.layer.removeInfluence "Use current layer offset"
tool.layer.removeInfluence.help "This option is only active when there is a current layer selected.

It will use the current layer offset to limit the displacement over strokes."
tool.layer.noLayerSelected "This option is only available if a current layer is selected"
// flatten
tool.flatten.planeLock "Lock plane"
// smooth
tool.smooth.stickyBorder "Sticky vertex on border"
// masking
tool.mask.clear "Clear"
tool.mask.invert "Invert"
tool.mask.blur "Blur"
tool.mask.sharpen "Sharpen"
tool.mask.thickness "Shell thickness"
tool.mask.polish "Border smoothness"
tool.mask.extract "Extract"
tool.mask.split "Split"
tool.mask.closeMask "Closing action (masked):"
tool.mask.closeUnmask "Closing action (unmasked):"
tool.mask.closeAction "Closing action:"
tool.mask.closeActionNone "None"
tool.mask.closeActionFill "Fill"
tool.mask.closeActionShell "Shell"
tool.mask.closeAction.help "-- None
Simply extract the part and let the extracted part opened.

-- Fill
Hole is filled and smoothed.
Do not use this option for flat surface.

-- Shell
Close the extracted shape by using the thickness value."
// matrix
tool.matrix "Matrix"
tool.matrix.clone "Clone"
tool.matrix.action "Action"
tool.matrix.action.help "-- Move origin
Move the mesh to the world origin.

-- Reset
Reset the mesh transform to identity.

-- Bake
Apply the matrix to the vertex and reset the matrix. Visually, nothing should change."
tool.matrix.transformOperation "Transform operation"
tool.matrix.translation "Translation"
tool.matrix.rotation "Rotation"
tool.matrix.scale "Scale"
tool.matrix.uniformScale "Uniform scale"
tool.matrix.uniformScale.help "Nomad cannot support non-uniform scale as object transform, so it will be applied as a vertex transformation."
tool.matrix.moveToOrigin "Move origin"
tool.matrix.resetTransform "Reset"
tool.matrix.bakeTransform "Bake"
tool.matrix.bakeTransform.confirm "The transform will be baked in the active layer.

Do you confirm?"
tool.matrix.applyMethod "Method:"
tool.matrix.applyMethodAuto "Auto"
tool.matrix.applyMethodVertex "Vertex"
tool.matrix.applyMethodObject "Object"
tool.matrix.applyMethod.help "-- Auto
Let Nomad choose between Vertex or Object mode.
Typically, object is preferred unless symmetry is enabled or if there is masking on the mesh.

-- Vertex
Vertices are transformed individually.
Symmetry and mask are taken into account.
For primitives that are not validated, Object mode is forced.

-- Object
The object is transformed as a whole.
Symmetry and mask are ignored.
If you use non-uniform scaling, Vertex mode will be forced."
// transform
tool.transform.multiTouch "Multi-touch"
tool.transform.multiTouch.help "If this option is disabled, you can only use one mode (translate, rotate, scale) at a time."
// gizmo
tool.gizmo.size "Widget size"
tool.gizmo.autoHide "Hide on interaction"
tool.gizmo.tap "Move custom pivot on single-tap"
tool.gizmo.tap.help "This option is only effective in custom pivot mode (Auto disabled).

-- None
Nothing happen when tapping on the mesh.

-- First hit
Move the gizmo on the first intersection.

-- Middle stab
Move the gizmo on the average of the first two intersections."
tool.gizmo.tapNone "None"
tool.gizmo.tapFirstHit "First hit"
tool.gizmo.tapMiddleStab "Middle stab"
// trim
tool.hole "Hole filling"
tool.hole.fillHoles "Fill holes"
// tool.hole.reproject "Reproject filled holes"
// tool.hole.reproject.help "Try to reproject the filled hole so that it follows more closely the cut.
// However, it will only work for rather simple projection."
tool.hole.bridges "Screen-space boolean"
tool.hole.bridges.help "If this option is enabled, you can punch holes in the volume.
The cut slope will also follow more closely the cutting shape."
tool.hole.threshold "Threshold epsilon"
tool.hole.threshold.help "Tweaking this value might help with the hole filling algorithm."
tool.hole.smoothing "Hole smoothing"
// smudge
// tool.smudge.projectScreen 
// tool.smudge.projectScreen.help "Smudge relies heavily on polygon density.\n
// Use this option if you want consistent smudge performance by projecting only once at the beginning of the stroke."
tool.smudge.fullProject "Project once"
tool.smudge.fullProject.help "You can make the smudge stroke faster by projecting the mesh only once at the beginning of the stroke.

If you don't move the camera between your smudge strokes, the projection can be avoided as well.

This setting is ignored if dynamic topology is activated."
tool.smudge.quality "Quality"
tool.smudge.quality.help "It changes the resolution of the projected pixels, lower values means faster strokes."
// trim / split / project / selMask
tool.shape "Shape"
tool.shape.rectangleSquare "Square"
tool.shape.rectangleCentered "Centered"
tool.shape.ellipseCircle "Circle"
tool.shape.ellipseCentered "Centered"
tool.shape.lineRotateStep "Rotate step"
// measure
tool.measure.goldenRatio "Show golden ratio"
// fallback
tool.noSettings "This tool doesn't have any specific settings."

// ------------------------------------------------------
// tool presets
tool.preset.save "Save"
tool.preset.reset "Reset"
tool.preset.clone "Clone"
tool.preset.delete "Remove"

// ------------------------------------------------------
// topology
topology "Topology"
// multires
topology.multires.title "Multiresolution"
topology.multires.title.help "Keep multiple resolution of a mesh.

If you make changes in a lower resolution, details from the higher resolutions will be reprojected when you switch back.

Layers are available on every resolution."
topology.multiresReverse "Reverse"
topology.multiresReverse.confirm "Could not create base subdivision.

The current topoloy is probably not a result from a subdivision."
topology.multiresReverse.confirm.yes "ok"
topology.multiresReverse.confirm.cancel ""
topology.multiresSubdivide "Subdivide"
topology.multiresSubdivideConfirm "The mesh will have $0M vertices, are you sure?"
topology.multiresDeleteLower "Delete lower"
topology.multiresDeleteHigher "Delete higher"
topology.multiresKeepTriangles "Keep triangles"
topology.multiresLinear "Flat subdivision"
// voxel
topology.voxel.title "Voxel remeshing"
topology.voxel.title.help "Remeshing by sampling the mesh on a grid.

If the object is not closed (watertight), an hole-filling algorithm will be applied first.

Layers are reprojected after remeshing but the quality will degrade."
topology.voxelResolution "Resolution"
topology.voxelRemesh "Remesh"
// dynamic topology
topology.surfaceUniform "Remesh"
topology.surfaceDetail "Detail"
topology.surfaceDetail.help "Unlike voxel remeshing, surface remeshing doesn't require the mesh to be closed.

It can also support masking so that you can protect some part of the mesh from topology changes.

Layers are updated correctly."
topology.surfaceMethod "Method"
toplogy.surfaceMethodUniformisation "Uniformisation"
toplogy.surfaceMethodSubdivision "Subdivision"
toplogy.surfaceMethodDecimation "Decimation"
topology.surfaceMethod.help "Behavior of dynamic topology:
- Uniformisation: add and remove detail
- Subdivision: add detail
- Decimation: remove detail"
topology.surfaceUseMasking "Protect masked area"
topology.surfaceUseMasking.help "The masked areas will protect the topology from beging changed."
topology.surfaceExtrapolate "Vertex extrapolation"
// dynamic
topology.dynamic "Dynamic topology"
topology.dynamicActivate "Enabled"
topology.dynamicActivate.help "With dynamic topology, sculpting tools can subdivide or simplify the mesh locally in real time.

This feature can have a noticeable impact on performance.

Layers are updated correctly."
topology.dynamicDetailLevel "Detail"
topology.dynamicDetailEdge "Detail"
topology.dynamicDetailMethod "Level of detail based on..."
topology.dynamicDetailMethodZoom "Zoom"
topology.dynamicDetailMethodRadius "Radius"
topology.dynamicDetailMethodConstant "Constant"
topology.dynamicDetailMethod.help "-- Zoom
The level of detail is based on how far you are from the surface.

-- Radius
The tool radius defines the amount of detail.

-- Constant
The detail is fixed, the detail value is shared with the voxel slider as well."
topology.dynamicQuality "Prefer..."
topology.dynamicQuality.help "If you choose Quality, the 2 main differences are:
- refinement is applied before the sculpting operator, you will get less interpolating artefact when painting or sculpting very small details
- refinement is not applied incrementally, if you sculpt very small details or do quick strokes, the topology will always be correctly refined

For better performance, and if you plan on using this option, you might consider enabling the \"partial drawing\" option in the Settings panel."
topology.dynamicQualitySpeed "Speed"
topology.dynamicQualityQuality "Quality"
topology.dynamicUsePressure "Use pressure on radius"
topology.dynamicUsePressure.help "Use this option if you want the pen pressure impact on tool radius to impact the level of detail."
topology.dynamicBrush "Brush"
topology.dynamicGlobal "Global"
topology.dynamicSettings "Settings - Brush / Global"

// ------------------------------------------------------
// version trial
version.buyWeb "Web version is only a demo"
version.buyFull "Upgrade to full version"
version.trialLimit "Trial version:
- 3 undo/redo possible
- 1 layer per mesh
- 1 active project only
- no import/export"
version.restorePurchase "Restore purchase"
version.fullFeatures "- Unlimited undo/redo
- Unlimited layers
- Save & Load
- Export & import"
