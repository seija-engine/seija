use core.math;


vec4 fxaa_console(vec2 uv,texture2D mainTexture,sampler texSampler)
{
    ivec2 texSize = textureSize(mainTexture,0);
    vec2 texelSize = 1.0 / texSize;
    vec4 origin = texture(sampler2D(mainTexture,texSampler),uv);
    float m = luminance(origin);
	  float nw = luminance(texture(sampler2D(mainTexture,texSampler),uv + vec2(-texelSize.x,  texelSize.y) * 0.5));
    float ne = luminance(texture(sampler2D(mainTexture,texSampler),uv + vec2(texelSize.x,  texelSize.y) * 0.5));
    float sw = luminance(texture(sampler2D(mainTexture,texSampler),uv + vec2(-texelSize.x,  -texelSize.y) * 0.5));
    float se = luminance(texture(sampler2D(mainTexture,texSampler),uv + vec2(texelSize.x,  -texelSize.y) * 0.5));

    float maxLuma = max(max(nw, ne), max(sw, se));
	float minLuma = min(min(nw, ne), min(sw, se));
	float contrast = max(maxLuma, m) -  min(minLuma, m);
    float contrastThreshold = 0.0312; //0.0312 - 0.0833
    float relativeThreshold =  0.063; //0.063 - 0.333
    //如果对比度值很小，认为不需要进行抗锯齿，直接跳过抗锯齿计算
	if(contrast < max(contrastThreshold, maxLuma * relativeThreshold))
	{
		return origin;
	}
    ne += 1.0 / 384;
    vec2 dir;
	dir.x = -((nw + ne) - (sw + se));
	dir.y = ((ne + se) - (nw + sw));
	dir = normalize(dir);
    vec2 dir1 = dir * texelSize * 0.5;
    vec4 n1 = texture(sampler2D(mainTexture,texSampler),uv - dir1);
    vec4 p1 = texture(sampler2D(mainTexture,texSampler),uv + dir1);
    vec4 result = (n1 + p1) * 0.5;
    float dirAbsMinTimesC = min(abs(dir1.x), abs(dir1.y)) * 8;

    vec2 dir2 = clamp(dir1.xy / dirAbsMinTimesC, -2.0, 2.0) * 2;
    vec4 n2 = texture(sampler2D(mainTexture,texSampler),uv - dir2 * texelSize);
    vec4 p2 = texture(sampler2D(mainTexture,texSampler),uv + dir2 * texelSize);
    vec4 result2 = result * 0.5 + (n2 + p2) * 0.25;
    float newLum = luminance(result2);
    if(newLum >= minLuma && newLum <= maxLuma) {
        result = result2;
    }
    return result;
}

/*
FXAA Quality的实现有问题
vec4 fxaa_fs_main_old(TexVSOutput o) {
  
  ivec2 texSize = textureSize(tex_mainTexture,0);
  vec2 texelSize = 1.0 / texSize;
  vec4 m = colorByOffset(0,0,texelSize,o.uv);
  vec4 w = colorByOffset(-1,0,texelSize,o.uv);
  vec4 e = colorByOffset(1,0,texelSize,o.uv);
  vec4 n = colorByOffset(0,1,texelSize,o.uv);
  vec4 s = colorByOffset(0,-1,texelSize,o.uv);

  vec4 sw = colorByOffset(-1,-1,texelSize,o.uv);
  vec4 ne = colorByOffset(1,1,texelSize,o.uv);
  vec4 nw = colorByOffset(-1,1,texelSize,o.uv);
  vec4 se = colorByOffset(1,-1,texelSize,o.uv);
  
  float lm = lumaColor(m);
  float lw = lumaColor(w);
  float le = lumaColor(e);
  float ln = lumaColor(n);
  float ls = lumaColor(s);
 
  float lsw = lumaColor(sw);
  float lne = lumaColor(ne);
  float lnw = lumaColor(nw);
  float lse = lumaColor(se);
  float maxLuma = max(max(max(max(lm,lw),le),ln),ls);
  float minLuma = min(min(min(min(lm,lw),le),ln),ls);
  float contrastThreshold = 0.0312; //0.0312 - 0.0833
  float relativeThreshold =  0.063; //0.063 - 0.333
  float contrast =  maxLuma - minLuma;
  //如果对比度值很小，认为不需要进行抗锯齿，直接跳过抗锯齿计算
	if(contrast < max(contrastThreshold, maxLuma * relativeThreshold))
	{
			return m;
	}

  // 先计算出锯齿的方向，是水平还是垂直方向
	float vertical   = abs(ln + ls - 2 * lm) * 2 + abs(lne + lse - 2 * le) + abs(lnw + lsw - 2 * lw);
	float horizontal = abs(le + lw - 2 * lm) * 2 + abs(lne + lnw - 2 * ln) + abs(lse + lsw - 2 * ls);
	bool isHorizontal = vertical > horizontal;
  //混合的方向
	vec2 pixelStep = isHorizontal ? vec2(0, texelSize.y) : vec2(texelSize.x, 0);
  

  // 确定混合方向的正负值
	float positive = abs((isHorizontal ? ln : le) - lm);
	float negative = abs((isHorizontal ? ls : lw) - lm);
  if(positive < negative) pixelStep = -pixelStep;
  //算出锯齿两侧的亮度变化的梯度值
  float gradient = 0;
  float oppositeLuminance = 0;
	if(positive > negative) {
		gradient = positive;
    oppositeLuminance = isHorizontal ? ln : le;
   
	} else {
		pixelStep = -pixelStep;
		gradient = negative;
    oppositeLuminance = isHorizontal ? ls : lw;
	}

  //这部分是基于亮度的混合系数计算
	float filterNumber = 2 * (ln + le + ls + lw) + lne + lnw + lse + lsw;
	filterNumber = filterNumber / 12;
	filterNumber = abs(filterNumber -  lm);
	filterNumber = min(max(1,filterNumber / contrast),0);
  // 基于亮度的混合系数值
	float pixelBlend = smoothstep(0, 1, filterNumber);
	pixelBlend = pixelBlend * pixelBlend;

  // 下面是基于边界的混合系数计算
	vec2 uvEdge = o.uv;
	uvEdge += pixelStep * 0.5;
	vec2 edgeStep = isHorizontal ? vec2(texelSize.x, 0) : vec2(0, texelSize.y);

  // 沿着锯齿边界两侧，进行搜索，找到锯齿的边界
	float edgeLuminance = (lm + oppositeLuminance) * 0.5;
	float gradientThreshold = gradient * 0.25;

  float pLuminanceDelta, nLuminanceDelta, pDistance, nDistance;
  int addIndex = 1;
  for(int index = 1; index <= 5; ++index) {
    vec4 texColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),uvEdge + index * edgeStep);
    pLuminanceDelta = lumaColor(texColor) - edgeLuminance;
    if(abs(pLuminanceDelta) > gradientThreshold) {
			  pDistance = index * (isHorizontal ? edgeStep.x : edgeStep.y);
			  break;
		}
    addIndex = addIndex + 1;
  }
  if(addIndex == 5) {
			pDistance = (isHorizontal ? edgeStep.x : edgeStep.y) * 8;
	}

  addIndex = 1;
  for(int index = 1; index <= 5; ++index) {
    vec4 texColor = texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),uvEdge - index * edgeStep);
    nLuminanceDelta = lumaColor(texColor) - edgeLuminance;
    if(abs(nLuminanceDelta) > gradientThreshold) {
			  nDistance = index * (isHorizontal ? edgeStep.x : edgeStep.y);
			  break;
		}
    addIndex = addIndex + 1;
  }
  if(addIndex == 5) {
			nDistance = (isHorizontal ? edgeStep.x : edgeStep.y) * 8;
	}

  float edgeBlend;
  // 这里是计算基于边界的混合系数，如果边界方向错误，直接设为0，如果方向正确，按照相对的距离来估算混合系数
	if (pDistance < nDistance) {
				if(sign(pLuminanceDelta) == sign(lm - edgeLuminance)) {
			        edgeBlend = 0;
			    } else {
			        edgeBlend = 0.5 - pDistance / (pDistance + nDistance);
			    }
	} else {
			    if(sign(nLuminanceDelta) == sign(lm - edgeLuminance)) {
			        edgeBlend = 0;
			    } else {
			        edgeBlend = 0.5 - nDistance / (pDistance + nDistance);
			    }
	}

  //从两种混合系数中，取最大的那个
	float finalBlend = max(pixelBlend, edgeBlend);
	vec4 result =  texture(sampler2D(tex_mainTexture,tex_mainTextureSampler),o.uv + pixelStep * finalBlend);
  return result;//vec4(isHorizontal?1:0,isHorizontal?0:1,0,1);
}
*/