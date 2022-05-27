// Comments and entries are sync from the english version, it's not
// possible to have language specific comments at the moment.
// You can use this entry to make a specific comment
language_note ""

// comments with "ICON FIT" should be short, ideally < 10 characters

// When in doubt, leave an empty string, it will fallback to english
// Some terms should probably be left untranslated
// For sure: Voxel, Matcap, DynTopo, PBR, Dyntopo
// Not sure: Roughness/Metalness? Mesh? Sub? tool names? etc

// ----------------------------------------------
// general stuffs

// Popup question, confirm? [yes/cancel] [ok/cancel] [delete/cancel]
confirm ""
yes ""
ok ""
delete ""
cancel ""

// feature: Auto / Off / On
on ""
off ""
auto ""

// coordinate
X ""
Y ""
Z ""

advancedSettings ""
notSaved ""

// generic warning when there is no mesh selected
noSelectedMesh ""

// generic warning when only one mesh needs to be selected
multipleObjectWarning ""

// ----------------------------------------------

// when you launch the app and there is missing Nomad/data files
loading.reprocess ""

// main pbr channel
baseColor ""
roughness ""
metalness ""

// ----------------------------------------------
// about
about.minify ""
about.minify.help ""
about.turntable ""
about.turntableSpeed ""
about.credits ""
about.creditsOpenSource ""
about.creditsArts ""
about.languages ""
about.languages.help ""
about.openUrl ""
// nomad
about.website ""
about.forum ""
about.manual ""
about.mail ""
// social
about.twitter ""
about.instagram ""
about.facebook ""
about.discord ""

// ----------------------------------------------
// alert
alert.hole.nothing ""
alert.shape.notVisible ""
alert.trim.nothing ""
alert.trim.full  ""
alert.mask.noExtract ""
alert.mask.noSplit ""
alert.view.disabled ""
alert.view.disabled.widgetPrimitive ""
alert.separate.fail ""
alert.voxelRemesh.success ""
alert.voxelRemesh.empty ""
alert.voxelRemesh.invalidInput ""
alert.matrix.clone ""
alert.gizmo.usePivot ""
alert.gizmo.useAuto ""
alert.gizmo.editPivot ""
alert.gizmo.editObject ""
alert.dynamic.enable ""
alert.dynamic.disable ""
alert.colorPicker ""
alert.backgroundTransform ""
alert.camera.resetView ""
alert.camera.snapView ""
alert.mask.show ""
alert.mask.hide ""
alert.selection.lock ""
alert.selection.unlock ""
alert.selection.isolate ""
alert.selection.showAll ""
alert.quickSave ""
alert.forceShowPainting.fill ""
alert.forceShowPainting.tool ""
alert.multiresLost ""
alert.rangeWarning ""
// autosave popup
alert.autoSave.auto ""
// bottom warning
alert.warning.needLayer ""
alert.warning.paintingHidden ""
alert.warning.noPartialWireframe ""
// bottom tip
alert.tip.shapeOrthographic ""
// undo
alert.state.trial ""

// ----------------------------------------------
// background
background ""
background.color ""
background.environment ""
background.blur ""
background.exposure ""

background.imageEnable ""
background.imageOverlay ""
background.imageAlpha ""
background.imageReset ""
background.imageTransform ""
// transform
background.imageX ""
background.imageY ""
background.imageRotation ""
background.imageScale ""

// ----------------------------------------------
// camera
camera ""
// saved views
camera.updateView ""
camera.addView ""
camera.focusOn ""
// projection
camera.projection ""
camera.orthographic ""
camera.perspective ""
camera.fov ""
camera.focal ""
// orbit
camara.orbit ""
camera.orbit.help ""
camera.trackball ""
camera.turntable ""
// speed
camera.speed ""
camera.rotation ""
camera.panning ""
camara.zooming ""
// misc
camera.resetView ""
camera.snapView ""
// interaction
camera.pivot ""
camera.doubleTapMesh ""
camera.doubleTapBackground ""
camera.doubleTapPivot ""
camera.doubleTapPivot.help ""
camera.airPivot ""
camera.airPivot.help ""
camera.autoPivot ""
camera.autoPivot.help ""
camera.doubleTapFocus ""
camera.doubleTapFocus.help ""
camera.doubleTapFocusSelection ""
camera.doubleTapFocusSelection.help ""

// toolbox context, only a few tools are display in some cases
// (solo visible en el modo de caja de herramientas expandida)
context.multiselection ""
context.triplanar ""
context.primitive ""

// scene and layer lists
curve.preset ""
curve.custom ""

// ----------------------------------------------
// debug
debug.uvPrimitive.warning ""
debug.uvPrimitive ""
debug.uvPrimitive.help ""
debug.uvNormalize ""
debug.uvNormalize.help ""
debug.uvBFF ""
debug.uvBFF.help ""
debug.logs ""
debug.heightmap ""
debug.graphics ""
debug.thumbnails ""

// scene and layer lists
expandList ""
expandList.help ""

// ----------------------------------------------
// file
file.project.empty ""
file.project.unsaved ""
file.project.loseUnsaved ""
file.project.lastManualSave ""
file.project.trialNoOpen ""
file.project.trialOnlyOpen ""

// ----------------------------------------------
// project
file.project ""
file.project.save ""
file.project.save.confirm ""
file.project.saveAs ""
file.project.saveAs.confirm ""
file.project.open ""
file.project.open.confirm ""
file.project.add ""
file.project.add.confirm ""
file.project.new ""
file.project.new.confirm ""
file.project.delete ""
file.project.delete.confirm ""
file.project.delete.confirmActive ""
file.project.delete.confirmOk ""
file.project.rename ""

// autosave
file.project.autoSave ""
file.project.autoSave.confirm ""
file.project.autoSave.help ""
file.project.autoSave.popup ""
file.project.autoSave.minutes ""
file.project.autoSave.delete ""
file.project.autoSave.delete.confirm ""

// import
file.import.title ""
file.import.title.help ""
file.importOpen ""
file.importOpen.confirm ""
file.import.add ""
file.import.add.confirm ""

file.exportSelection ""
file.exportSelection.help ""
file.convertToQuad ""
file.convertToQuad.help ""

// export
file.export.title ""
file.export.title.help ""

// generic export
file.export.texture ""
file.export.texture.help ""
file.export.normal ""
file.export.normal.help ""

// gltf
file.export.gltf ""
file.export.gltfLayer ""
file.export.gltfLayer.help ""
file.export.gltfLayerPaint ""
file.export.gltfLayerPaint.help ""
file.export.gltfLayerNomad ""
file.export.gltfLayerNomad.help ""
file.export.gltfColor0 ""
file.export.gltfColor0.help ""
file.export.gltfColor1 ""
file.export.gltfColor1.help ""

// obj
file.export.obj ""
file.export.objWarning ""
file.export.objColorAppend ""
file.export.objColorAppend.help ""

// stl
file.export.stl ""
file.export.stlWarning ""
file.export.stlColor ""
file.export.stlColor.help ""
file.export.stlAscii ""

file.settings.title ""
file.settings.title.help ""

// settings
file.settings.reset ""
file.settings.reset.confirm ""

// materials
file.materials ""
file.materials.reset ""
file.materials.reset.confirm ""

// tools
file.herramienta ""
file.tools.reset ""
file.tools.reset.confirm ""

// render
file.render ""
file.render.showInterface ""
file.render.renderRatio ""
file.render.renderRatio.help ""
file.render.help ""
file.render.size ""
file.render.size.custom ""
file.render.screenResolution ""
file.render.export ""
file.render.width ""
file.render.height ""
file.render.warn ""
file.render.transparent ""
file.render.transparent.help ""

// ----------------------------------------------
// gesture menu
gesture.useGlobal ""
gesture.useGlobal.help ""

gesture.pressure ""
gesture.pressureTitle ""
gesture.pressure.noTool ""
gesture.pressure.noGrab ""
gesture.pressure.radius ""
gesture.pressure.intensity ""
gesture.pressure.useRadius ""
gesture.pressure.useIntensity ""
gesture.pressure.curveRadius ""
gesture.pressure.curveIntensity ""

gesture.cameraInteraction ""
gesture.sculptInteraction ""
gesture.interaction.fingerAndStylus ""
gesture.interaction.finger ""
gesture.interaction.stylus ""

gesture.fingerLighting ""
gesture.fingerLighting.help ""
gesture.fingerRadius ""
gesture.fingerRadius.help ""

gesture.fingerSmooth ""
gesture.unknownPressure ""
gesture.unknownPressure.help ""

// pencil
gesture.pencilAction.none ""
gesture.pencilAction.smooth ""
gesture.pencilAction.alt ""
gesture.pencilAction.android ""
gesture.pencilAction.android.help ""
gesture.pencilAction.ios ""
gesture.pencilAction.ios.help ""

// history
gesture.history ""
gesture.history.help ""

// size rejection
gesture.useSizeRejection ""
gesture.useSizeRejectionConfirm ""
gesture.useSizeRejection.help ""
gesture.sizeRejection ""
// help
gesture.interaction.title ""
gesture.interaction.title.help ""

// ----------------------------------------------
// history
history ""
history.root ""
history.undoConfirm ""
history.undoWarning ""
history.stack ""
history.limitSize ""
history.limitSize.help ""
history.limitStack ""
history.limitStack.help ""
history.rangeProtect ""
history.rangeProtect.help ""
history.restoreCamera ""
history.restoreCamera.help ""
// display undo/redo
history.state.undo ""
history.state.redo ""
history.state.symmetrySplit ""
history.state.voxelRemesh ""
history.state.surfaceRemesh ""
// state multires
history.state.multiresToDynamic ""
history.state.multiresLevel ""
history.state.multiresSubdivide ""
history.state.multiresReverse ""
history.state.multiresDeleteLower ""
history.state.multiresDeleteHigher ""
// mesh
history.state.meshDynamicToStatic ""
history.state.meshStaticToDynamic ""
history.state.meshSymmetryUpdate ""
history.state.meshMatrixUpdate ""
history.state.meshVisibility ""
history.state.meshMaterial ""
// state scene
history.state.sceneAddRemove ""
history.state.sceneMeshOrder ""
// state layer
history.state.layerOrder ""
history.state.layerMergeRedo ""
history.state.layerCreate ""
history.state.layerDelete ""
history.state.layerMerge ""
history.state.layerHide ""
history.state.layerShow ""
history.state.layerSelect ""
history.state.layerUnselect ""
history.state.layerName ""
history.state.layerFactor ""
history.state.layerFactorOffset ""
history.state.layerFactorColor ""
history.state.layerFactorRoughness ""
history.state.layerFactorMetalness ""
// state light
history.state.lightVisible ""
history.state.lightIntensity ""
history.state.lightColor ""
history.state.lightPosition ""
history.state.lightShadow ""
history.state.lightShadowBias ""
history.state.lightAttachment ""
history.state.lightAdd ""
history.state.lightDelete ""
history.state.lightCopy ""
history.state.lightMove ""
history.state.lightType ""
history.state.lightSpotAngle ""
history.state.lightSpotSoftness ""
// state view
history.state.viewAdd ""
history.state.viewMove ""
history.state.viewDelete ""

// ----------------------------------------------
// interface
interface ""

// bottom buttons
interface.bottomButtons ""

// colors
interface.colors ""
interface.colorSelect ""
interface.colorBase ""
interface.colorBaseTransparent ""
interface.panelTransparent ""
interface.blurFactor ""

// color preset
interface.colorsPresets ""
interface.presetBlurRed ""
interface.presetBlurBlue ""
interface.presetBlurGreen ""
interface.presetBlurYellow ""
interface.presetBlackWhite ""
interface.presetWhiteBlack ""
interface.presetLividOrange ""
interface.presetCardboard ""
interface.presetDefault ""

// style
interface.style ""
interface.resetAll ""
interface.resetAll.confirm ""
interface.flipTop ""
interface.flipBottom ""
interface.flipMiddle ""
interface.showTooltips ""
interface.showTooltips.help ""
interface.materialPreview ""
interface.toolboxHide ""
interface.toolboxHide.help ""
interface.toolboxMaxColumn ""
interface.toolboxResetOrder ""
interface.rounding ""
interface.curveToolSymmetric ""
interface.curveToolSymmetric.help ""
interface.scale ""
interface.cursorStep ""
interface.panelWidth ""
interface.fontScale ""

// ----------------------------------------------
// layer sub menu
layer.action ""
layer.name ""
layer.delete ""
layer.move ""
layer.duplicate ""
layer.mergeDown ""
layer.factors ""
layer.offsetFactor ""
layer.colorFactor ""

// ----------------------------------------------
// layers menu
layers.addLayer ""
layers.addLayerTrial ""
layers.title ""
layers.title.help ""
layers.primitive ""
layers.baseSelected ""

// ----------------------------------------------
// light sub menu
light ""
light.intensity ""
light.attachment ""
light.attachment.fixed ""
light.attachment.camera ""
light.attachment.environment ""
light.attachment.help ""
light.type ""
light.type.directional ""
light.type.spot ""
light.type.point ""
light.spotAngle ""
light.spotSoftness ""
light.shadowCast ""
light.shadowNormalBias ""
light.visible ""
light.resetPosition ""

// ----------------------------------------------
// material
material ""
material.addNew ""
// if the shading mode is in matcap or unlit
material.pbrRoughnessMetalnessWarning ""
material.pbrReflectanceWarning ""
material.pbrRefractionWarning ""
material.pbrSubsurfaceWarning ""
// refraction
material.ior ""
material.paintingOverride ""
material.paintingOverride.help ""
material.refractionSurfaceGlossiness ""
material.refractionSurfaceGlossiness.help ""
material.refractionInteriorRoughness ""
material.refractionInteriorRoughness.help ""
material.paintGlossy ""
material.paintGlossy.help ""
// absorption
material.absorptionEnable ""
material.absorptionEnable.help ""
material.absorptionFactor ""
// subsurface
material.subsurfaceDepth ""
material.translucency ""
material.translucency.help ""
// type
material.opacity ""
material.type.opaque ""
material.type.subsurface ""
material.type.subsurface.help ""
material.type.blending ""
material.type.blending.help ""
material.type.additive ""
material.type.additive.help ""
material.type.dithering ""
material.type.dithering.help ""
material.type.refraction ""
material.type.refraction.help ""
// shadows
material.castShadows ""
material.receiveShadows ""
// backface
material.twoSided ""
material.alwaysUnlit ""
material.flipCulling ""
// material
material.reflectance ""
material.reflectance.help ""

// ----------------------------------------------
// menu name (visible on small screen menu are collapsed)
menu.files ""
menu.scene ""
menu.multires ""
menu.voxel ""
menu.dyntopo ""
menu.topology ""
menu.primitive ""
menu.render ""
menu.material ""
menu.postProcess ""
menu.camera ""
menu.background ""
menu.tool ""
menu.stroke ""
menu.paint ""
menu.symmetry ""
menu.pressure ""
menu.layers ""
menu.settings ""
menu.interface ""
menu.history ""
menu.historySettings ""
menu.about ""
menu.debug ""

// ----------------------------------------------
// mesh sub menu
mesh.action ""
mesh.holeClose ""
mesh.holeDetail ""
mesh.separate ""
mesh.triplanarWarning ""
mesh.triplanarResolution ""
mesh.triplanarCubic ""
mesh.triplanarConvert ""
mesh.name ""
mesh.type ""
mesh.typeStatic ""
mesh.typeMultiresolution ""
mesh.typeDynamic ""

// ----------------------------------------------
// painting
paint.useGlobal ""
paint.useGlobal.help ""
paint.usePainting ""
paint.intensity ""
paint.paintAll ""
paint.paintAll.help ""
paint.paintAllForce ""
paint.paintAllForce.help ""
paint.strokePaintingTitle ""
paint.layerWarning ""
paint.texture.title ""
paint.texture.title.help ""
paint.texture.warningEnable ""
paint.texture.warningIgnored ""
paint.useAlpha ""
paint.useAlpha.help ""
paint.useFalloff ""
paint.useFalloff.help ""

// ----------------------------------------------
// popup (for example tap on a tool, to open edit popup)
popup.save ""
popup.save.confirm ""
popup.lastSave ""
popup.lastSave.confirm ""
popup.reset ""
popup.reset.confirm ""
popup.clone ""
popup.rename ""
popup.delete ""
popup.delete.confirm ""
popup.delete.confirm.yes ""

// title when requesting input value through virtual keyboard
input.name ""
input.number ""
input.hexcolor ""

// ----------------------------------------------
// postprocess
postprocess.mainEnable ""
postprocess.quality ""
postprocess.quality.help ""
postprocess.maxSamples ""
postprocess.fullResolution ""
postprocess.renderRatio ""
postprocess.renderRatioWarning ""
postprocess.renderRatio.help ""
// fxaa
postprocess.fxaaEnable ""
// taa
postprocess.taaEnable ""
postprocess.taaEnable.help ""
// ssr
postprocess.ssrEnable ""
postprocess.ssrPBRWarning ""
// ssao
postprocess.ssaoEnable ""
postprocess.ssaoRadius ""
postprocess.ssaoFactor ""
postprocess.ssaoBias ""
postprocess.ssaoBias.help ""
// dof
postprocess.dofEnable ""
postprocess.dofBlurFar ""
postprocess.dofBlurNear ""
postprocess.dofFocusTip ""
// bloom
postprocess.bloomEnable ""
postprocess.bloomIntensity ""
postprocess.bloomRadius ""
postprocess.bloomRadius.help ""
postprocess.bloomThreshold ""
postprocess.bloomThreshold.help ""
// tone mapping
postprocess.toneEnable ""
postprocess.toneExposure ""
postprocess.toneContrast ""
postprocess.toneSaturation ""
postprocess.toneMappingNone ""
// curve
postprocess.curveEnable ""
postprocess.curve.luminance ""
postprocess.curve.red ""
postprocess.curve.green ""
postprocess.curve.blue ""
postprocess.curveReset ""
postprocess.curveResetAll ""
// chromatic
postprocess.chromaticEnable ""
postprocess.chromaticFactor ""
// vignette
postprocess.vignetteEnable ""
postprocess.vignetteSize ""
postprocess.vignetteHardness ""
// sharpness
postprocess.sharpnessEnable ""
postprocess.sharpnessFactor ""
// grain
postprocess.grainEnable ""
postprocess.grainFactor ""
// curvature
postprocess.curvatureEnable ""
postprocess.curvatureCavity ""
postprocess.curvatureBump ""
// pixelart
postprocess.pixelArtEnable ""
// scanline
postprocess.scanlineEnable ""
postprocess.scanlineFactor ""
postprocess.scanlineSpacing ""

// ----------------------------------------------
// primitive (scene menu)
primitive ""
primitive.box ""
primitive.sphereCube ""
primitive.sphereUV ""
primitive.icosahedron ""
primitive.cylinder ""
primitive.cone ""
primitive.torus ""
primitive.lathe ""
primitive.tube ""
primitive.plane ""
primitive.triplanar ""
primitive.faceXYZ ""
primitive.faceXYZ.help ""
primitive.needValidate ""

// for 3d editing in viewport
primitive.useFloatPanel ""
primitive.useFloatPanel.help ""
primitive.edit ""
primitive.edit.help ""

primitive.mainConfig ""
primitive.topology ""
primitive.geometry ""

// common config
primitive.validate ""
primitive.maxFaces ""
primitive.maxFaces.help ""
primitive.linear ""
primitive.subdivision ""

// common config
primitive.radius ""
primitive.size ""
primitive.sizeX ""
primitive.sizeY ""
primitive.sizeZ ""
primitive.division ""
primitive.divisionX ""
primitive.divisionY ""
primitive.divisionZ ""
primitive.angleX ""
primitive.angleY ""
primitive.angleZ ""
primitive.constantDensity ""
primitive.projectOnSphere ""
primitive.projectOnSphere.help ""

// triplanar
primitive.triplanar.title ""
primitive.triplanar.title.help ""
primitive.triplanarSameSize ""
primitive.triplanarPolish ""
primitive.triplanarResetMask ""
primitive.triplanarReproject ""
primitive.triplanarReproject.title ""
primitive.isolate.all ""
primitive.isolate.back ""
primitive.isolate.right ""
primitive.isolate.bottom ""
// plane
primitive.planeSameSize ""
primitive.planeDisk ""
// box
primitive.boxRegular ""
// tube
primitive.tubeSnapOffset ""
primitive.tubeSnapOffset.help ""
primitive.tubeThicknessStart ""
primitive.tubeThicknessEnd ""
// primitive.tubeTwist "Giro"
// primitive.tubeTwistRotate "Rotación"
// primitive.tubeTwistRadius "Magnitud"
// primitive.tubeTwistOffset "Desplazamiento"
primitive.tubeSnap ""
// lathe
// torus
primitive.torusRadiusOuter ""
primitive.torusRadiusInner ""
primitive.torusAngle ""
primitive.torusAngleOffset ""
// cylinder
primitive.cylinderHeight ""
// cone
primitive.coneRadius ""
primitive.coneHeight ""
// hole sub menu (cylinder, tube, etc)
primitive.hole ""
primitive.hasHole ""
// both used for hole radius and main radius
primitive.radiusSync ""
primitive.radiusStart ""
primitive.radiusEnd ""
primitive.editRadius ""
// spline (for lathe and tube)
primitive.spline ""

// common resources stuffs
resource.delete ""
resource.import ""

// ----------------------------------------------
// scene
scene.title ""
scene.title.help ""
scene.mergeSimple ""
scene.mergeVoxel ""
scene.voxelResolution ""
scene.subtractionTip ""
scene.intersectionTip ""

// ----------------------------------------------
// settings
settings.displayTitle ""
// wireframe
settings.wireframeTitle ""
settings.wireframeDisplay ""
settings.debugUV ""
settings.debugUV.help ""
// backface
settings.backfaceTitle ""
settings.backfaceVisible ""
settings.backfaceVisible.help ""
settings.backfaceColor ""
settings.backfaceColored ""
// outline
settings.outlineTitle ""
settings.outlineEnable ""
settings.outlineThickness ""
// snap cube
settings.snapCubeTitle ""
settings.snapCubeDisplay ""
settings.snapCubeBottom ""
settings.snapCubeLeft ""
// stats
settings.statsTitle ""
settings.statsDisplay ""
settings.statsRight ""
settings.statsAll ""
// grid
settings.gridTitle ""
settings.gridDisplay ""
// cursor
settings.cursorWhileSculpting ""
settings.cursorShowDot ""
settings.cursorShowDot.help ""
settings.cursorShowRope ""
// highlight
settings.highlightSelectionTitle ""
settings.highlightSelection ""
settings.highlightDuration ""
// other
settings.darkenUnselected ""
settings.smoothShading ""
settings.partialDraw ""
settings.partialDraw.help ""
settings.partialDrawWarning ""
settings.showPainting ""
settings.lightIcon ""
settings.lightIcon.help ""
settings.holeTitle ""
settings.holeNonManifold ""
settings.holeNonManifold.help ""
settings.loadGuiSettings ""
settings.loadGuiSettings.help ""
settings.loadObjSplitByGroup ""
settings.loadObjSplitByGroup.help ""
settings.loadMergeLayers ""
settings.loadSkipTextures ""
settings.loadKeepTopology ""
settings.loadKeepTopology.help ""
settings.loadReverseVertices ""
settings.loadReverseVertices.help ""
// multires
settings.multiresTitle ""
settings.multiresMaxVertices ""
settings.multiresMaxVertices.help ""
settings.multiresLowResVertices ""
settings.multiresLowResVertices.help ""
// experimental
settings.experimentalTitle ""

// ----------------------------------------------
// shading
shading ""
// main render mode
shading.pbr ""
shading.pbr.help ""
shading.matcap ""
shading.matcap.help ""
shading.unlit ""
shading.unlit.help ""
// textures
shading.textures ""
shading.textures.help ""
// lights
shading.lights ""
shading.lights.addLight ""
shading.lights.pbrWarning ""
// environment
shading.environment ""
shading.environmentImport ""
shading.environmentExposure ""
shading.environmentBackgroundBlur ""
shading.environmentRotation ""
shading.environmentRotation.help ""
shading.environmentAttachedToCamera ""
shading.environmentAttachedToCamera.help ""
// matcap
shading.matcap ""
shading.matcapRotation ""
shading.matcapRotation.help ""
shading.matcapGlobal ""
shading.matcapGlobal.help ""

// ----------------------------------------------
// bottom shortcut buttons (ICON FIT)
shortcut.maskVisible ""
shortcut.solo ""
shortcut.voxelRemesh ""
shortcut.wireframe ""
shortcut.cameraReset ""
shortcut.cameraSnap ""
shortcut.lockSelection ""
shortcut.perspective ""
shortcut.grid ""
shortcut.uv ""

// can be longer (customization name in Interface menu)
shortcut.voxelRemesh.long ""
shortcut.wireframe.long ""
shortcut.cameraReset.long ""
shortcut.cameraSnap.long ""
shortcut.lockSelection.long ""
shortcut.lockSelection.long.help ""
shortcut.perspective.long ""
shortcut.grid.long ""

// ----------------------------------------------
// stat
stat.ramScene ""
stat.vramScene ""
stat.vramRender ""
stat.vramTextures ""
stat.ramHistory ""
stat.ramOther ""
stat.usedMemory ""
stat.freeMemory ""
stat.ram ""
stat.used ""
stat.free ""
stat.faces ""
stat.triangles ""
stat.vertices ""
stat.quads ""
stat.sceneFaces ""
stat.sceneVertices ""

// ----------------------------------------------
// stroke
stroke ""
strokeTitle ""
stroke.useWorldRadius ""
stroke.useWorldRadius.help ""
stroke.useShareRadius ""
stroke.useShareRadius.help ""
stroke.minSpacingAdjustIntensity ""
stroke.minSpacingAdjustIntensity.help ""
stroke.minSpacing ""
stroke.minSpacing.help ""
stroke.lazySmooth ""
stroke.lazySmooth.help ""
stroke.lazyRadius ""
stroke.lazyRadius.help ""
stroke.globalSettings ""
stroke.snapRadius ""
stroke.snapRadius.help ""
stroke.sculptOffset ""
stroke.sculptOffset.help ""
stroke.accumulate ""
stroke.accumulate.help ""
stroke.useDynamicTopology ""
stroke.connectedTopology ""
stroke.connectedTopology.help ""
stroke.onlyFrontFace ""
stroke.onlyFrontFace.help ""
stroke.onlySameSide ""
stroke.onlySameSide.help ""
stroke.curveFalloff ""
stroke.onlyLasso ""
// alpha
stroke.alpha ""
stroke.alphaInvert ""
stroke.alphaWrap ""
stroke.alphaWrap.none ""
stroke.alphaWrap.repeat ""
stroke.alphaWrap.mirror ""
stroke.alphaProject ""
stroke.alphaProject.surfaceContinuous ""
stroke.alphaProject.screenFixed ""
stroke.alphaTiling ""
stroke.alphaScale ""
stroke.alphaScale.help ""
stroke.alphaMidValue ""
stroke.alphaMidValue.help ""
// stroke type
stroke.strokeType ""
stroke.strokeTypeDot ""
stroke.strokeTypeDrag ""
stroke.strokeTypeGrab ""
stroke.strokeTypeGrabRadius ""
stroke.strokeTypeGrabIntensity ""

// ----------------------------------------------
// symmetry
symmetry ""
symmetry.enable ""
symmetry.plane.title ""
symmetry.toolIgnore ""
symmetry.radial.title ""
symmetry.radialX ""
symmetry.radialY ""
symmetry.radialZ ""
// method
symmetry.method ""
symmetry.method.help ""
symmetry.methodWorld ""
symmetry.methodLocal ""
// flip
symmetry.flip ""
// mirror
symmetry.mirror ""
symmetry.mirror.help ""
symmetry.mirrorLeftToRight ""
symmetry.mirrorRightToLeft ""
symmetry.mirrorFail ""
symmetry.mirrorUseMasking ""
symmetry.mirrorUseMasking.help ""
// reset
symmetry.reset ""
symmetry.resetCenterMesh ""
symmetry.resetCenterWorld ""
symmetry.resetDirection ""
// advanced
symmetry.showLine ""
symmetry.showPlane ""
symmetry.editWarning ""
symmetry.edit ""
symmetry.edit.help ""

// ----------------------------------------------
// tools icons on the left bar (ICON FIT)
tool.dynTopo ""
tool.symmetry ""
tool.mirror ""
tool.clay ""
tool.clay.sub ""
tool.brush ""
tool.move ""
tool.move.normal ""
tool.drag ""
tool.smooth ""
tool.smooth.relax ""
tool.mask ""
tool.mask.unmask ""
tool.maskSelector ""
tool.smudge ""
tool.flatten ""
tool.flatten.fill ""
tool.layer ""
tool.crease ""
tool.trim ""
tool.split ""
tool.project ""
tool.inflate ""
tool.pinch ""
tool.nudge ""
tool.stamp ""
tool.clearLayer ""
tool.lassoSelect ""
tool.gizmo ""
tool.gizmo.auto ""
tool.gizmo.editPivot ""
tool.gizmo.rotateSnap ""
tool.gizmo.local ""
tool.transform ""
tool.transform.move ""
tool.transform.rotate ""
tool.transform.scale ""
tool.transform.snap ""
tool.measure ""
tool.view ""
tool.lathe ""
tool.tube ""
tool.insert ""
tool.shape.flip ""
tool.shape.view ""
tool.shape.lasso ""
tool.shape.curve ""
tool.shape.polygon ""
tool.shape.path ""
tool.shape.rectangle ""
tool.shape.ellipse ""
tool.shape.line ""
tool.shape.closed ""

// popup when editing sliders
tool.sliderRadius ""
tool.sliderIntensity ""

// ----------------------------------------------
// title
tool.settingsTitle ""

// ----------------------------------------------
// tool menu
tool.noSettings ""

// ----------------------------------------------
// clay
tool.clay.flattenOffset ""

// ----------------------------------------------
// crease
tool.crease.pinchFactor ""

// ----------------------------------------------
// layer
tool.layer.removeInfluence ""
tool.layer.removeInfluence.help ""
tool.layer.noLayerSelected ""

// ----------------------------------------------
// flatten
tool.flatten.warning ""
tool.flatten.planeLockOrigin ""
tool.flatten.planeLockNormal ""
tool.flatten.planeAverageOrigin ""
tool.flatten.planeAverageNormal ""
tool.flatten.planeOffset ""

// ----------------------------------------------
// smooth
tool.smooth.stickyBorder ""

// ----------------------------------------------
// paint
tool.paint ""
tool.paint.erase ""
tool.paint.depthFilter ""
tool.paint.layerFilter ""
tool.paint.layerFilter.help ""

// ----------------------------------------------
// masking
tool.mask.clear ""
tool.mask.invert ""
tool.mask.flipConnected ""
tool.mask.blur ""
tool.mask.sharpen ""
tool.mask.thickness ""
tool.mask.polish ""
tool.mask.engraveEmboss ""
tool.mask.extract ""
tool.mask.split ""
tool.mask.closeMask ""
tool.mask.closeUnmask ""
tool.mask.closeAction ""
tool.mask.closeActionNone ""
tool.mask.closeActionFill ""
tool.mask.closeActionShell ""
tool.mask.closeActionLayer ""
tool.mask.closeAction.help ""

// ----------------------------------------------
// matrix (transform / gizmo)
tool.matrix ""
tool.matrix.clone ""
tool.matrix.action ""
tool.matrix.action.help ""
tool.matrix.transformOperation ""
tool.matrix.translation ""
tool.matrix.rotation ""
tool.matrix.scale ""
tool.matrix.uniformScale ""
tool.matrix.uniformScale.help ""
tool.matrix.moveToOrigin ""
tool.matrix.resetTransform ""
tool.matrix.bakeTransform ""
tool.matrix.applyMethod ""
tool.matrix.applyMethodAuto ""
tool.matrix.applyMethodVertex ""
tool.matrix.applyMethodObject ""
tool.matrix.applyMethod.help ""

// ----------------------------------------------
// transform
tool.transform.multiTouch ""
tool.transform.multiTouch.help ""

// ----------------------------------------------
// gizmo
tool.gizmo.size ""
tool.gizmo.linearRollThreshold ""
tool.gizmo.linearRollThreshold.help ""
tool.gizmo.autoHide ""
tool.gizmo.tap ""
tool.gizmo.tap.help ""
tool.gizmo.tapNone ""
tool.gizmo.tapFirstHit ""
tool.gizmo.tapMiddleStab ""

// ----------------------------------------------
// lathe
tool.lathe.axis ""
tool.lathe.axis.fixed ""
tool.lathe.axis.dynamic ""

// ----------------------------------------------
// tube
tool.tube.snap ""
tool.tube.snap.all ""
tool.tube.snap.startEnd ""

// ----------------------------------------------
// trim
tool.hole ""
tool.hole.fillHoles ""
// tool.hole.reproject "Reproyectar agujeros rellenos"
// tool.hole.reproject.help "Intenta volver a proyectar el agujero rellenado para que sigas más de cerca el corte.
// Sin embargo, solo funcionará para una proyección bastante simple."
tool.hole.bridges ""
tool.hole.bridges.help ""
tool.hole.threshold ""
tool.hole.threshold.help ""
tool.hole.smoothing ""

// ----------------------------------------------
// smudge
tool.smudge.quality ""
tool.smudge.quality.help ""

// ----------------------------------------------
// trim / split / project / selMask
tool.shape ""
tool.shape.rectangleSquare ""
tool.shape.rectangleCentered ""
tool.shape.ellipseCircle ""
tool.shape.ellipseCentered ""
tool.shape.lineRotateStep ""

// ----------------------------------------------
// measure
tool.measure.goldenRatio ""

// ----------------------------------------------
// topology
topology ""
// multires
topology.multires.title ""
topology.multires.title.help ""
topology.multiresReverse ""
topology.multiresReverse.confirm ""
topology.multiresSubdivide ""
topology.multiresSubdivideConfirm ""
topology.multiresDeleteLower ""
topology.multiresDeleteHigher ""
topology.multiresKeepTriangles ""
topology.multiresLinear ""
topology.multiresLinear.help ""
// voxel
topology.voxel.title ""
topology.voxel.title.help ""
topology.voxelResolution ""
topology.voxelRemesh ""
topology.voxelSharp ""
topology.voxelSharp.help ""
topology.voxelSubLevel ""
topology.voxelSubLevel.help ""
// dynamic topology
topology.surfaceUniform ""
topology.surfaceDetail ""
topology.surfaceDetail.help ""
topology.surfaceMethod ""
toplogy.surfaceMethodUniformisation ""
toplogy.surfaceMethodSubdivision ""
toplogy.surfaceMethodDecimation ""
topology.surfaceMethod.help ""
topology.surfaceUseMasking ""
topology.surfaceUseMasking.help ""
topology.surfaceExtrapolate ""
// dynamic
topology.dynamic ""
topology.dynamicActivate ""
topology.dynamicActivate.help ""
topology.dynamicDetailLevel ""
topology.dynamicDetailEdge ""
topology.dynamicDetailMethod ""
topology.dynamicDetailMethodZoom ""
topology.dynamicDetailMethodRadius ""
topology.dynamicDetailMethodConstant ""
topology.dynamicDetailMethod.help ""
topology.dynamicQuality ""
topology.dynamicQuality.help ""
topology.dynamicQualitySpeed ""
topology.dynamicQualityQuality ""
topology.dynamicUsePressure ""
topology.dynamicUsePressure.help ""
// topology.dynamicBrush "Brocha"
// topology.dynamicGlobal "Global"
// topology.dynamicSettings "Configuración - Brocha / Global"
// decimate
topology.decimate.title ""
topology.decimate.title.help ""
topology.decimate ""
topology.decimateTargetFaces ""
topology.decimatePaintWeight ""
topology.decimatePaintWeight.help ""
topology.decimateUniform ""
topology.decimateUniform.help ""
// topology.decimatePreserveBorders "Preservar bordes"
// topology.decimatePreserveBorders.help "No diezmes el borde de la malla.

// This is only relevant for object that are not watertight."

// BFF is activated through Debug menu
topology.uv.title ""
topology.uvAtlas ""
topology.uvAtlas.warning ""
topology.uvBFF ""
topology.uvBFF.warning ""
topology.uvBFFCones ""
topology.uvBFFCones.help ""
topology.uvDelete ""

// baking
topology.bake ""
topology.bake.help ""
topology.bakeResolution ""

// ----------------------------------------------
// privacy policy
privacyPolicy.title ""
privacyPolicy.reject ""
privacyPolicy ""

// ----------------------------------------------
// version trial
version.buyWeb ""
version.buyFull ""
version.restorePurchase ""

version.trialHistory ""
version.trialLayer ""
version.trialOneProject ""
version.trialNoImport ""
version.trialNoExport ""

version.fullFeatures ""
