// comments with "ICON FIT" means < 10 characters
// arguments with $0 $1 etc

// general stuffs
yes "确认"
ok "好"
cancel "取消"
delete "删除"
X "X"
Y "Y"
Z "Z"
noSelectedMesh "对象未选择"
advancedSettings "高级设置"

// pbr
baseColor "颜色"
roughness "粗糙度"
metalness "金属强度"

// ------------------------------------------------------
// about
about.minify "全屏显示"
about.minify.help "在设备支持的情况下，可以通过四指触碰屏幕来开关全屏显示"
about.turntable "旋转展示"
about.turntableSpeed "旋转速度"
about.credits "鸣谢"
about.creditsOpenSource "开源项目"
about.creditsArts "MatCap与HDRI"

// ------------------------------------------------------
// alert
alert.confirmDelete "请确认是否删除？"
alert.confirmDelete.yes "确认删除"
alert.hole.nothing "该对象没有孔洞！"
alert.shape.notVisible "当前网格不可见！"
alert.trim.nothing "未找到可裁切对象"
alert.trim.full  "对象不能完全裁切"
alert.mask.noExtract "未找到可提取对象"
alert.mask.noSplit "未找到可分离对象"
alert.view.disabled "一些功能将在浏览模式下禁用："
alert.separate.fail "该对象只有一部分，所以无法分开"
alert.voxelRemesh.success "网格重构成功！"
alert.voxelRemesh.empty "网格重构失败，因为结果并未产生面。"
alert.voxelRemesh.invalidInput "输入无效！"
alert.matrix.clone "The object will be duplicated"
alert.gizmo.usePivot "使用自定义坐标原点"
alert.gizmo.useAuto "使用自动坐标原点"
alert.gizmo.editPivot "编辑坐标原点模式"
alert.gizmo.editObject "编辑对象模式"
alert.dynamic.enable "启用动态网格"
alert.dynamic.disable "关闭动态网格"
alert.colorPicker "在网格上拖动手指选取一个颜色"
alert.camera.resetView "重置视图"
alert.camera.snapView "切换视图"
alert.mask.show "显示蒙版"
alert.mask.hide "隐藏蒙版"
alert.selection.lock "锁定所选项"
alert.selection.unlock "解锁所选项"
alert.selection.isolate "隔离所选项"
alert.selection.showAll "显示全部"
alert.quickSave "正在自动保存..."
alert.multiresLost "模型细分将会丢失，是否继续？"
// autosave popup
alert.autoSave.auto "将在 $0s 后自动保存"
// bottom warning
alert.warning.needLayer "当前工具仅在活动图层上可用"
alert.warning.multiresLost "模型细分将会丢失"
alert.warning.paintingHidden "绘画已被隐藏，请在设置面板里将其打开。"
alert.warning.noPartialWireframe "打开线框显示时，局部绘画将被禁用。"
// bottom tip
alert.tip.polygonDot "轻点绿色图标以应用几何体。"
alert.tip.shapeOrthographic "为了避免因透视视图而产生的偏差，建议在相机设置里切换到正交视图。"
// undo
alert.state.trial "这是试用版本，您无法再撤销。"

// ------------------------------------------------------
// background
background "背景"
background.settings "设置"
background.color "颜色"
background.environment "环境"
background.blur "模糊"
background.exposure "曝光"

background.imageEnable "参考图像"
background.imageEnable "启用"
background.imageX "X轴方向"
background.imageY "Y轴方向"
background.imageRotation "旋转"
background.imageScale "缩放"
background.imageOverlay "透明度"
background.imageReset "重设"

// ------------------------------------------------------
// camera
camera "相机"
// saved views
camera.updateView "更新视图？"
camera.addView "添加视图"
camera.focusOn "正在观察"
// projection
camera.projection "视图"
camera.orthographic "正交视图"
camera.perspective "透视视图"
camera.fov "焦距"
// orbit
camera.orbit "视图旋转"
camera.orbit.help "旋转模式启用后可使用双指旋转视图。"
camera.trackball "旋转模式"
camera.turntable "水平模式"
// speed
camera.speed "相机速度"
camera.rotation "旋转"
camera.panning "平移"
camera.zooming "缩放"
// misc
camera.resetView "重置视图"
camera.snapView "固定视图"
// interaction
camera.pivot "视图中心点"
camera.doubleTapMesh "双击对象"
camera.doubleTapBackground "双击背景"
camera.doubleTapPivot "双击后改变"
camera.doubleTapPivot.help "双击后改变坐标视图中心点。"
camera.autoPivot "平移/缩放后改变"
camera.autoPivot.help "双指移动相机后，中心点会随之移动。"
camera.doubleTapFocus "聚焦"
camera.doubleTapFocus.help "双击物体表面后视图中心将移动至该点。"
camera.doubleTapFocusSelection "聚焦所选项"
camera.doubleTapFocusSelection.help "双击背景后相机将会缩放移动至最适合该对象的视图。"

// scene and layer lists
expandList "展开图标"
expandList.help "可以让菜单里的图标排列间距放大。"

// ------------------------------------------------------
// file
file.project.empty "您没有保存的项目"
file.project.unsaved "更改未保存！"
file.project.loseUnsaved "如不保存，您的更改将会丢失！"
file.project.lastManualSave "上一次手动保存的预览"
file.project.trialNoOpen "这是试用版本，您将无法重新打开当前项目！"
file.project.trialOnlyOpen "这是试用版本，您只能打开当前项目！"

// project
file.project "项目"
file.project.save "保存"
file.project.save.confirm "确认保存 $0？"
file.project.saveAs "另存为"
file.project.saveAs.confirm "确认覆盖 $0？"
file.project.open "打开"
file.project.open.confirm "将打开选定的项目 $0？"
file.project.add "添加"
file.project.add.confirm "确认添加 $0 至当前项目？"
file.project.new "新建"
file.project.new.confirm "确认新建场景？"
file.project.delete "删除"
file.project.delete.confirm "确认删除 $0？"
file.project.delete.confirmActive "删除 $0？\n\n这是当前正打开的项目！"
file.project.delete.confirmOk "确定要删除吗？"

// autosave
file.project.autoSave "自动保存项目"
file.project.autoSave.help "每隔一段时间将您的项目另存为一个单独的文件。

这个自动保存文件可以在以下目录找到：

$0"
file.project.autoSave.popup "弹窗提醒"
file.project.autoSave.minutes "自动保存间隔"
file.project.autoSave.delete "删除自动保存文件"
file.project.autoSave.delete.confirm "确认删除？"

// import
file.import.title "从外部导入"
file.import.title.help "支持导入的格式：\n- Wavefront (.obj)\n- glTF 2.0 (.glb .gltf)\n- STL (.stl)"
file.importOpen "打开"
file.importOpen.confirm "确定打开新文件？"
file.import.add "添加"
file.import.add.confirm "确定添加新文件？"

file.exportSelection "只导出选择部分"
file.exportSelection.help "只导出当前选择的网格，而不是所有场景。"
file.convertToQuad "Reconstruct quad"
file.convertToQuad.help "Reconstruct quads from triangles by pairing triangle (if they are adjacent in the files)."

// export
file.export.title "导出"
file.export.title.help "建议导出 glTF 格式，因为它比其他格式支持更多属性。"

// gltf
file.export.gltf "导出 glTF 2.0"
file.export.gltfLayer "导出图层"
file.export.gltfLayer.help "将图层导出为可变体。这是来自官方的特性，能在更多软件上使用。"
file.export.gltfColor "导出颜色"
file.export.gltfColor.help "导出的是顶点颜色。这是来自官方的特性，能在更多软件上使用。"
file.export.gltfNormal "导出法线"
file.export.gltfNormal.help "如想在其他软件上打开该文件，请勾选此选项。\n该选项对本软件没有影响。"
file.export.gltfExtraPaint "导出其他"
file.export.gltfExtraPaint.help "将导出粗糙度、金属强度、蒙版和图层绘画。其他软件不会读取该属性。"

// obj
file.export.obj "导出 OBJ 格式"
file.export.objWarning "图层、粗糙度、金属强度、蒙版和绘画图层等其他属性将会丢失。"
file.export.objColorAppend "导出颜色"
file.export.objColorAppend.help "在顶点之后添加颜色信息\n\n只有部分3D软件能够识别。"
file.export.objColorHexa "十六进制颜色"
file.export.objColorHexa.help "像ZBrush那样将颜色转换为十六进制。\n\n只有部分3D软件能够识别。"

// stl
file.export.stl "导出 STL 格式"
file.export.stlWarning "图层和绘画将会丢失"
file.export.stlAscii "默认情况下，格式为二进制。\n\n您可以选择导出为文本格式（ASCII），但文件会更大。"

file.settings.title "设置"
file.settings.title.help "大部分应用的设置都保存在此处（相机界面等）。\n
某些资源将自动保存在其他地方，包括：
- HDR
- 材质
- 画笔形状
- 背景
- 项目\n
目前暂时无法保存画笔设置，自定义笔画已在开发计划中。"

// settings
file.settings.reset "恢复默认设置"
file.settings.reset.confirm "确定重设所有设置？\n\n项目、画笔形状、材质、HDRI与背景将不会被影响。"

// materials
file.materials "材质库"
file.materials.reset "重置为默认"
file.materials.reset.confirm "确定要重置材质库吗？"

// tools
file.tools "工具预设"
file.tools.reset "重置为默认"
file.tools.reset.confirm "确定要重置材质库吗？"

// render
file.render "渲染"
file.render.showInterface "显示操作界面"
file.render.size "渲染尺寸"
file.render.screenResolution "屏幕尺寸"
file.render.export "导出为png"
file.render.4kWarn "导出4K格式可能会使用大量内存，请确认文件保存之后再导出！"
file.render.transparent "导出透明背景"
file.render.transparent.help "打开此选项可以让您更方便地把渲染图导入到平面软件里。\n
暂不支持部分透明对象导出。"

// ------------------------------------------------------
// history
history "历史记录"
history.root "新建"
history.undoConfirm "您确定要撤销所有操作吗？"
history.undoWarning "如在此之后进行更改，将会覆盖之前的所有操作。"
history.stack "历史记录设置"
history.limitSize "历史记录限制 (Mb)"
history.limitSize.help "历史记录的最大大小（以Mb为单位）。\n
历史记录状态会随着下一个操作记录而改变。"
history.limitStack "历史记录步数"
history.limitStack.help "程序可保留的最大操作数量。\n
历史记录状态会随着下一个操作记录而改变。"
history.rangeProtect "历史记录保护范围"
history.rangeProtect.help "如您在历史记录中做了大量操作，程序会在覆盖操作之前提示您。"
history.gesture "快捷手势"
history.gesture.help "双指轻点撤销。\n\n三指轻点重做。"
history.restoreCamera "恢复相机视角"
history.restoreCamera.help "启用该选项后您可以在撤销或重做时同时恢复当时的相机视角。"
// display undo/redo
history.state.undo "撤销： $0"
history.state.redo "重做： $0"
history.state.symmetrySplit "镜像"
history.state.voxelRemesh "体素网格重构"
history.state.surfaceRemesh "表面网格重构"
// state multires
history.state.multiresToDynamic "模型细分转为动态网格"
history.state.multiresLevel "改变分辨率"
history.state.multiresSubdivide "细分网格"
history.state.multiresReverse "粗化网格"
history.state.multiresDeleteLower "删除低分辨率模型"
history.state.multiresDeleteHigher "删除高分辨率模型"
// mesh
history.state.meshDynamicToStatic "动态网格转为静态网格"
history.state.meshStaticToDynamic "静态网格转为动态网格"
history.state.meshSymmetryUpdate "改变对称"
history.state.meshMatrixUpdate "矩阵变换"
history.state.meshVisibility "可见性"
history.state.meshMaterial "改变材质"
// state scene
history.state.sceneAddRemove "场景"
history.state.sceneMeshOrder "模型顺序"
// state layer
history.state.layerOrder "更改图层顺序 $0"
history.state.layerMergeRedo "取消合并图层 $0"
history.state.layerCreate "添加图层 $0"
history.state.layerDelete "删除图层 $0";
history.state.layerMerge "合并图层 $0";
history.state.layerHide "隐藏图层 $0"
history.state.layerShow "显示图层 $0"
history.state.layerSelect "选择图层 $0"
history.state.layerUnselect "取消选择图层 $0"
history.state.layerName "图层 $0 重命名";
history.state.layerFactor "调整图层 $0 参数";
history.state.layerFactorOffset "调整图层 $0 偏移度";
history.state.layerFactorColor "调整图层 $0 透明度";
history.state.layerFactorRoughness "调整图层 $0 粗糙度";
history.state.layerFactorMetalness "调整图层 $0 金属强度";
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

// ------------------------------------------------------
// pressure menu
input.useGlobal "使用全局压感设置"
input.useGlobal.help "勾选后，所有工具都会使用相同的压感参数。\n
如您希望给此工具单独设定压感参数，请取消勾选"

input.pressure "压感"
input.pressureTitle "压感设置 ($0)"
input.pressure.noTool "此工具不适用压感设置。"
input.pressure.noGrab "此工具会忽略压感设置。"
input.pressure.radius "半径"
input.pressure.intensity "强度" 
input.pressure.useRadius "启用"
input.pressure.useIntensity "启用" 
input.pressure.curveRadius "半径"
input.pressure.curveIntensity "强度"

input.cameraInteraction "相机移动："
input.sculptInteraction "雕刻:"
input.interaction.fingerAndStylus "手指与触控笔"
input.interaction.finger "手指"
input.interaction.stylus "触控笔"

input.fingerSmooth "将手指用于平滑"
input.unknownPressure "允许未识别的压感"
input.unknownPressure.help "当您的触控笔压感无法使用或者希望使用手指压感时，请勾选此选项。" 
// pencil
input.pencilAction.none "无"
input.pencilAction.smooth "平滑"
input.pencilAction.alt "添加或减去"
input.pencilAction.android "触控笔按钮"
input.pencilAction.android.help "实验功能"
input.pencilAction.ios "双击Pencil"
input.pencilAction.ios.help "仅支持Apple Pencil 第二代"
// size rejection
input.useSizeRejection "启用忽略尺寸"
input.useSizeRejectionWarning "如果您的手指无法操控，请退出并重启Nomad。\n此选项每次重启后都会关闭。"
input.useSizeRejectionConfirm "请确保您的文件妥善保存后再点击确认！"
input.useSizeRejection.help "如果手指与屏幕的接触面积超过设定值，屏幕将忽略手指的本次操作。\n
部分设备可能不支持此选项"
input.sizeRejection "尺寸阈值"
// help
input.interaction.title "交互选项" 
input.interaction.title.help "以下选项均为全局设置。"

// ------------------------------------------------------
// interface
interface "界面设置"

// bottom buttons
interface.bottomButtons "底部按钮"
interface.quickVoxelRemesh "体素网格重构"
interface.quickWireframe "网格"
interface.quickLockSelection "锁定选择"
interface.quickLockSelection.help "启用后，您无法通过点击方式选择对象。"
interface.quickCameraReset "重置视图"
interface.quickCameraSnap "切换视图"
interface.quickCameraSnapFlip "翻转基本视图"
interface.quickCameraSnapFlip.help "当相机处于基本视图时，点击切换视图将会翻转至背面。"

// left
interface.leftButtons "左边按钮"
interface.quickSmooth "平滑"
interface.quickMask "蒙版"
interface.quickToggle "反向操作锁定"
interface.quickPaint "材质"
interface.quickAlpha "画笔形状"
interface.maskGesture "蒙版手势"
interface.screenTooSmall "If the screen is too small, some buttons won't be displayed."
interface.maskGesture.help "按住蒙版快捷方式，并且：\n
- 在背景上拖动可清除蒙版
- 在背景上点击可反相蒙版
- 在对象上点击可锐化蒙版边界"

// colors
interface.colors "界面颜色"
interface.colorSelect "主色"
interface.colorBase "底色"
interface.colorBaseTransparent "面板颜色" 
interface.panelTransparent "面板透明度"
interface.blurFactor "模糊强度"

// color preset
interface.colorsPresets "界面预设"
interface.presetBlurRed "红"
interface.presetBlurBlue "蓝"
interface.presetBlurGreen "绿"
interface.presetBlurYellow "黄"
interface.presetBlackWhite "黑与白"
interface.presetWhiteBlack "白与黑"
interface.presetLividOrange "青与橙"
interface.presetCardboard "纸板"
interface.presetDefault "默认设置"

// style
interface.style "菜单风格"
interface.resetAll "重置界面设置"
interface.resetAll.confirm "确定要重置界面吗？"
interface.flipTop "整体反转"
interface.flipBottom "反转底部图标"
interface.flipMiddle "反转中间图标"
interface.autoClose "操作时收起菜单"
interface.autoClose.help "点击背景与对象时自动收起菜单"
interface.showTooltips "显示工具提示"
interface.showTooltips.help "你在点的这个小问号就是工具提示 :-D"
interface.materialPreview "调整材质参数预览"
interface.toolboxHide "自动隐藏工具栏"
interface.toolboxHide.help "如果你想隐藏工具栏，请勾选此选项。"
interface.toolboxMaxColumn "工具栏列数"
interface.rounding "界面圆角"
interface.inlined "滑块紧凑"
interface.dampingSlider "降低滑块灵敏度"
interface.dampingSlider.help "勾选此选项可让滑块在调节参数时更加精确。"
interface.curveToolSymmetric "压力曲线对称"
interface.curveToolSymmetric.help "使画笔设置里的衰减参数曲线对称。"
interface.animated "动效"
interface.scale "界面缩放"
interface.cursorStep "垂直间距"
interface.panelWidth "面板宽度"
interface.fontScale "字体尺寸"

// toolbox
interface.toolsOrder "工具顺序"
interface.openOrderTools "编辑"
interface.resetOrderTools "重设"
interface.resetOrderTools.confirm "确认要重设顺序吗？"

// debug
interface.debug "Debugging"
interface.debug.warning "For debugging only!"

// ------------------------------------------------------
// layer sub menu
layer.action "操作"
layer.name "重命名"
layer.delete "删除图层"
layer.move "移动图层"
layer.duplicate "复制图层"
layer.mergeDown "向下合并"
layer.factors "通道参数"
layer.offsetFactor "位置偏移"
layer.colorFactor "颜色浓度"

// ------------------------------------------------------
// layers menu
layers.addLayer "添加图层"
layers.addLayerTrial "试用版本只能给每个对象添加一个图层！"
layers.title "图层"
layers.title.help "图层能够记录位置偏移和绘画，这对于非线性工作流程来说非常有用。
例如，通过试验不同的面部表情而不依赖于历史记录来撤消更改。

图层是从上到下排序的，所以上方的图层会遮盖下方的图层。

为了解决图层的不透明性，图层的所有通道（颜色浓度、粗糙度、金属强度）都会使用相同的蒙版。
您可以使用橡皮工具来擦除当前图层上的绘画蒙版。";
layers.multipleObjectWarning "您选择了多个对象，无法修改图层。"
layers.primitive "基本体无法添加图层。"
layers.baseSelected "无"

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
Light orientation won't change.\n
-- Camera
Light orientation depends on the camera view."
light.shadowCast "Shadow"
light.shadowNormalBias "Normal bias"

// ------------------------------------------------------
// material
material "材质"
material.addNew "添加新材质"
material.matcapWarning "粗糙度与金属强度在材质捕捉模式下不可用。"
material.opacity = "透明度"

// ------------------------------------------------------
// mesh sub menu
mesh.action "操作"
mesh.closeHoles "封闭孔洞"
mesh.separate "分离对象"
mesh.triplanarWarning "图层、绘画与模型细分将会丢失。"
mesh.triplanarResolution "分辨率"
mesh.triplanarCubic "强制转换为立方体"
mesh.triplanarConvert "转换"
mesh.name "名称"
mesh.type "类型"
mesh.typeStatic "静态模型"
mesh.typeMultiresolution "模型细分"
mesh.typeDynamic "动态模型"

// ------------------------------------------------------
// painting
paint.useGlobal "应用全局材料"
paint.useGlobal.help "如勾选此选项，其他工具的材质也将会与所选材质相同。"
paint.usePainting "启用" 
paint.useColor "颜色" 
paint.useRoughness "粗糙度" 
paint.useMetalness "金属强度"
paint.intensity "画笔强度"
paint.paintAll "全部上色" 
paint.paintAll.help "将当前材料应用到所选对象上。"
paint.paintAllForce "强制全部上色"
paint.paintAllForce.help "将当前材料应用到所选对象上。\n
蒙版区域与未勾选通道也会被应用。"
paint.strokePaintingTitle "画笔设置 ($0)"
paint.layerWarning "图层上的通道蒙版不可用。"

// ------------------------------------------------------
// postprocess
postprocess.mainEnable "后期处理" 
// fxaa
postprocess.fxaaEnable "抗锯齿（FXAA）"
// ssr
postprocess.ssrEnable "屏幕空间反射（SSR）" 
postprocess.ssrFactor "强度" 
postprocess.ssrDistanceFading "淡化距离" 
postprocess.ssrDistanceFading.help "根据反射距离来减弱效果
此选项能减弱SSR所产生的伪影。"
postprocess.ssrMatcapWarning "SSR仅在PBR渲染模式下有效。"
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
primitive.triplanar "三向投影"
primitive.needValidate "Primitives should be validated in order to be sculpted."

primitive.mainConfig "Parameter"
primitive.topology "Topology"
primitive.geometry "Geometry"

// common config
primitive.mirror "Mirroring"
primitive.mirror.help "Duplicate the primitiveb by using the symmetrical plane."
primitive.validate "Validate"
primitive.maxFaces "Max faces"
primitive.maxFaces.help "The maximum number of faces a primitive can have.\n
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
primitive.triplanar.title.help "Triplanar is using the mask information from 3 planes to fill a voxel grid that is then polygonized.\n
If you interact with the division or size sliders, the painting information will reset (smoothness is ok).\n
You should probably disable symmetry as it might not function as you would expect.\n
You can use the 'Topologically connected' option in the mask panel to paint a plane impacting the other planes."
primitive.triplanarIsolate "Visibility"
primitive.triplanarSameSize "Same size (cube)"
primitive.triplanarPolish "Smoothness"
primitive.triplanarResetMask "Reset mask"
primitive.triplanarReproject "Resize mask"
primitive.triplanarReproject.title "Reproject the plane mask when updating the triplanar settings.\n\
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
scene.title.help "By holding the Smooth shortcut you can:
- select multiple objects in the viewport
- range-select objects in the interface list"
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
settings.backfaceVisible.help "Backface faces are faces that point 'away' from the camera viewpoint.\n
All faces (triangle or quad) point to a certain direction, for example on a base sphere will see its faces point towards the outside.\n
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
settings.partialDraw.help "Feature not polished!\n
Use it if you are sculpting a relatively small part of a high poly mesh.\n
It should make the sculpting smoother, but you should not enable wireframe!\n
Also it might add visual artefacts during the brush strokes"
settings.partialDrawWarning "Do not forget to turn off this option if the visual artefacts are too bothersome!"
settings.detailRangeTitle "Voxel/Dynamic remesh"
settings.detailRange "Max detail range"
settings.detailRange.help "Maximum value for voxel and dynamic topology level of detail.\n
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
This option is not saved in the settings.\n"
settings.keepImportTopology "Keep topology at import"
settings.keepImportTopology.help "Use this option if you don't want Nomad to fiddle with the topology of imported mesh.\n
It will disable vertex/face reordering, removal of vertex/face duplicates and removal of unused vertices."
// multires
settings.multiresTitle "Multiresolution"
settings.multiresMaxVertices "Max vertices count"
settings.multiresMaxVertices.help "Nomad does not perform memory check before subdivision, high poly count can easily lead to crashes."
settings.multiresLowResVertices "Low resolution vertices threshold"
settings.multiresLowResVertices.help "A lower resolution of the mesh can be displayed when you move the camera.\n
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
shading "渲染模式"
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
shading.environmentAttachedToCamera.help "Attach the environment to the camera.\n
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
stroke.minSpacing.help "Spacing between each stroke, relative to the tool radius.\n
Lower value will allow smoother stroke but performance will degrade."
stroke.lazySmooth "Stroke smoothing"
stroke.lazySmooth.help "Average multiple pointer position to get a smoother stroke.\n
With high values, the stroke will lag behind the pointer but will eventually catch up."
stroke.lazyRadius "Lazy rope stabilizer"
stroke.lazyRadius.help "Strokes will lag behind the pointer position according to a certain distance.\n
This can be used to draw smooth lines."
stroke.globalSettings "This is a global setting"
stroke.snapRadius "Snap radius"
stroke.snapRadius.help "Snap the stroke if the pointer lies close to the last recorded stroke.\n
This can be useful when drawing long continuous lines, while doing frequent pauses."
stroke.sculptOffset "Stroke offset"
stroke.sculptOffset.help "Apply a constant offset on the stroke.\n
This option is there to help for small screen when using fingers, so that your finger doesn't cover the stroke."
stroke.accumulate "Accumulate stroke"
stroke.accumulate.help "If this option is enabled, there is no limit to how much matter you can add/remove per stroke."
stroke.useDynamicTopology "Allow dynamic topology"
stroke.connectedTopology "Connected topology"
stroke.connectedTopology.help "This option will only sculpt the vertices that are linked to the picked surface.\n
This is typically used for the Move tool, for example if you want to exclusively move a part that self-intersect with another part."
stroke.onlyFrontFace "Front-facing vertex only"
stroke.onlyFrontFace.help "This option will ignore back facing vertices.\n
It can be useful if you want to paint part of a thin geometry without impacting the other side.\n
It also works for sculpting but you might experience some artefacts."
stroke.intensityMultiplier "Intensity multiplier"
stroke.curveFalloff "Falloff"
stroke.onlyLasso "Settings only active for the lasso tool."
// alpha
stroke.alpha "Alpha" 
stroke.alphaInvert "Invert value"
stroke.alphaScale "Alpha scale"
stroke.alphaScale.help "At minimum value, the alpha square is inside the tool circle radius.\n
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
The symmetry plane will move along the mesh when you use one of the transform tools (rotate, translate or gizmo).\n

-- World
The symmetry plane is fixed and will not move."
symmetry.methodWorld "World"
symmetry.methodLocal "Local"
// mirror
symmetry.mirror "Mirroring"
symmetry.mirror.help "Try to re-apply the symmetry without impacting the topology.\n
To succeed, the topology need to be symmetrical and at least one edge should lie exactly on the symmetry line.\n
If it fails, you will be proposed to force the symmetry, but it will impact the topology."
symmetry.apply "Mirror"
symmetry.flip "Flip direction"
symmetry.flip.help "Use this option to change the side in which the details are projected."
symmetry.applyFail "Failed to apply symmetry:
- $0\n
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
symmetry.edit.help "You can freely set the symmetry plane.\n
This feature is a bit experimental and you should probably never use it."

// ------------------------------------------------------
// tools
// left bar generic (ICON FIT)
tool.sliderDegree "Rotate $0 °"
tool.sliderRadius "Radius $0 %"
tool.sliderIntensity "Intensity $0 %"
tool.dynTopo "DynTopo"
tool.symmetry "对称"
tool.mirror "镜像"
tool.clay "黏土"
tool.clay.sub "反向"
tool.brush "标准"
tool.move "移动"
tool.move.normal "法线方向"
tool.drag "拖拽"
tool.smooth "平滑"
tool.smooth.relax "规整网格"
tool.mask "蒙版"
tool.mask.unmask "消除蒙版"
tool.maskSelector "选择蒙版"
tool.paint "绘画"
tool.paint.erase "橡皮"
tool.smudge "涂抹"
tool.flatten "铲平"
tool.flatten.fill "填充"
tool.layer "层"
tool.crease "褶皱"
tool.trim "裁切"
tool.split "分割"
tool.project "投射"
tool.inflate "膨胀"
tool.pinch "挤捏"
tool.nudge "触碰"
tool.stamp "印记"
tool.clearLayer "擦除"
tool.gizmo "轴向变换"
tool.gizmo.auto "自动原点"
tool.gizmo.editPivot "编辑原点"
tool.gizmo.local "轴向"
tool.transform "自由变换"
tool.transform.move "移动"
tool.transform.rotate "旋转"
tool.transform.scale "缩放"
tool.measure "测量"
tool.view "浏览模式"
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
tool.layer.removeInfluence.help "This option is only active when there is a current layer selected.\n
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
tool.matrix.bakeTransform.confirm "The transform will be baked in the active layer.\n\nDo you confirm?"
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
tool.gizmo.tap.help "This option is only effective in custom pivot mode (Auto disabled).\n
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
tool.smudge.fullProject.help "You can make the smudge stroke faster by projecting the mesh only once at the beginning of the stroke.\n
If you don't move the camera between your smudge strokes, the projection can be avoided as well.\n
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
// topology
topology "Topology"
// multires
topology.multires.title "Multiresolution"
topology.multires.title.help "Keep multiple resolution of a mesh.\n
If you make changes in a lower resolution, details from the higher resolutions will be reprojected when you switch back.\n
Layers are available on every resolution."
topology.multiresReverse "Reverse"
topology.multiresReverse.confirm "Could not create base subdivision.\n
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
topology.voxel.title.help "Remeshing by sampling the mesh on a grid.\n
If the object is not closed (watertight), an hole-filling algorithm will be applied first.\n
Layers are reprojected after remeshing but the quality will degrade."
topology.voxelResolution "Resolution"
topology.voxelRemesh "Remesh"
// dynamic topology
topology.surfaceUniform "Remesh"
topology.surfaceDetail "Detail"
topology.surfaceDetail.help "Unlike voxel remeshing, surface remeshing doesn't require the mesh to be closed.\n
It can also support masking so that you can protect some part of the mesh from topology changes.\n
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
topology.dynamicActivate.help "With dynamic topology, sculpting tools can subdivide or simplify the mesh locally in real time.\n
This feature can have a noticeable impact on performance.\n
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
- refinement is not applied incrementally, if you sculpt very small details or do quick strokes, the topology will always be correctly refined\n
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
version.buyWeb "网页版仅供演示"
version.buyFull "购买完整版本"
version.trialLimit "试用版本限制：
- 仅允许3次以内的撤销或重做
- 每个物体仅允许添加一个图层
- 仅允许启用一个项目
- 不允许导入和导出"
version.restorePurchase "恢复购买"
version.fullFeatures "- 撤消或重做不受限制
- 图层数量不受限制
- 允许保存和载入
- 可以导入和导出文件"
