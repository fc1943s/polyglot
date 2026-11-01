program SpiralGenerated;
{$mode delphi}{$H+}
{$MAXSTACKSIZE $10000000}
uses SysUtils, Math;
type
  TMut0 = class;
  TMut1 = class;
  TMut2 = class;
  TMut3 = class;
  TArray0 = array of Double;
  TMut0 = class l0: LongInt; end;
  TArray1 = array of LongInt;
  TMut1 = class l0: Double; l1: Double; l2: Double; end;
  TMut2 = class l0: LongInt; end;
  TMut3 = class l0: Double; end;
function method0(v0: TMut0): Boolean; forward;
function method1(v0: TMut0): Boolean; forward;
function method4(v0: Double; v1: TMut3): Boolean; forward;
function method5(v0: Double; v1: TMut3): Boolean; forward;
procedure method6(v0: TArray0; v1: TArray1; v2: Double; v3: Double; v4: Double; v5: Double; v6: Double; v7: Double; v8: Double; v9: LongInt); forward;
procedure method3(v0: TArray0; v1: TArray1; v2: Double; v3: Double; v4: Double; v5: Double; v6: Double); forward;
function method7(v0: TMut0): Boolean; forward;
function method8(v0: TMut0): Boolean; forward;
function method2(v0: TArray0; v1: TArray1; v2: Double; v3: Double; v4: Double; v5: LongInt): LongInt; forward;
function MutCreate0(a0: LongInt): TMut0;
begin
  Result := TMut0.Create; Result.l0 := a0;
end;
function method0(v0: TMut0): Boolean;
var
  v1: LongInt;
  v2: Boolean;
begin
  v1 := v0.l0;
  v2 := v1 < 7040;
  Result := v2;
end;
function MutCreate1(a0: Double; a1: Double; a2: Double): TMut1;
begin
  Result := TMut1.Create; Result.l0 := a0; Result.l1 := a1; Result.l2 := a2;
end;
function MutCreate2(a0: LongInt): TMut2;
begin
  Result := TMut2.Create; Result.l0 := a0;
end;
function method1(v0: TMut0): Boolean;
var
  v1: LongInt;
  v2: Boolean;
begin
  v1 := v0.l0;
  v2 := v1 < 60;
  Result := v2;
end;
function MutCreate3(a0: Double): TMut3;
begin
  Result := TMut3.Create; Result.l0 := a0;
end;
function method4(v0: Double; v1: TMut3): Boolean;
var
  v2: Double;
  v3: Boolean;
begin
  v2 := v1.l0;
  v3 := v2 < v0;
  Result := v3;
end;
function method5(v0: Double; v1: TMut3): Boolean;
var
  v2: Double;
  v3: Boolean;
begin
  v2 := v1.l0;
  v3 := v2 < v0;
  Result := v3;
end;
procedure method6(v0: TArray0; v1: TArray1; v2: Double; v3: Double; v4: Double; v5: Double; v6: Double; v7: Double; v8: Double; v9: LongInt);
var
  v10: Double;
  v11: Double;
  v12: Double;
  v13: Double;
  v14: Double;
  v15: Double;
  v16: Double;
  v17: Double;
  v18: Double;
  v19: Double;
  v20: Double;
  v21: Double;
  v22: Double;
  v23: Double;
  v24: Double;
  v25: Double;
  v26: Double;
  v27: Double;
  v28: Double;
  v29: Double;
  v30: Double;
  v31: Double;
  v32: Double;
  v33: Double;
  v34: Double;
  v35: Double;
  v36: Double;
  v37: Double;
  v38: Double;
  v39: Double;
  v40: Double;
  v41: Double;
  v42: Double;
  v43: Double;
  v44: Double;
  v45: Double;
  v46: Double;
  v47: Double;
  v48: Double;
  v49: Double;
  v50: Double;
  v51: Double;
  v52: Double;
  v53: LongInt;
  v54: Double;
  v55: Double;
  v56: LongInt;
  v57: LongInt;
  v58: LongInt;
  v59: Boolean;
  v61: Boolean;
  v60: Boolean;
  v62: Double;
  v63: Boolean;
begin
  v10 := Sin(v2);
  v11 := v7 * v10;
  v12 := Sin(v3);
  v13 := v11 * v12;
  v14 := Cos(v4);
  v15 := v13 * v14;
  v16 := Cos(v2);
  v17 := v8 * v16;
  v18 := v17 * v12;
  v19 := v18 * v14;
  v20 := v15 - v19;
  v21 := v7 * v16;
  v22 := Sin(v4);
  v23 := v21 * v22;
  v24 := v20 + v23;
  v25 := v8 * v10;
  v26 := v25 * v22;
  v27 := v24 + v26;
  v28 := Cos(v3);
  v29 := v6 * v28;
  v30 := v29 * v14;
  v31 := v27 + v30;
  v32 := v21 * v14;
  v33 := v25 * v14;
  v34 := v32 + v33;
  v35 := v13 * v22;
  v36 := v34 - v35;
  v37 := v18 * v22;
  v38 := v36 + v37;
  v39 := v29 * v22;
  v40 := v38 - v39;
  v41 := v17 * v28;
  v42 := v11 * v28;
  v43 := v41 - v42;
  v44 := v6 * v12;
  v45 := v43 + v44;
  v46 := v45 + 100.0;
  v47 := 1.0 / v46;
  v48 := 80.0 + v5;
  v49 := 40.0 * v47;
  v50 := v49 * v31;
  v51 := v50 * 2.0;
  v52 := v48 + v51;
  v53 := LongInt(Trunc(v52));
  v54 := v49 * v40;
  v55 := 22.0 + v54;
  v56 := LongInt(Trunc(v55));
  v57 := v56 * 160;
  v58 := v53 + v57;
  v59 := v58 >= 0;
  if v59 then begin
      v60 := v58 < 7040;
      v61 := v60;
  end else begin
      v61 := False;
  end;
  if v61 then begin
      v62 := v0[v58];
      v63 := v47 > v62;
      if v63 then begin
          v0[v58] := v47;
          v1[v58] := v9;
      end else begin
      end;
  end else begin
  end;
end;
procedure method3(v0: TArray0; v1: TArray1; v2: Double; v3: Double; v4: Double; v5: Double; v6: Double);
var
  v7: Double;
  v8: TMut3;
  v10: TMut3;
  v12: Double;
  v13: Double;
  v14: LongInt;
  v15: LongInt;
  v16: Double;
  v17: LongInt;
  v18: LongInt;
  v19: Double;
  v20: LongInt;
  v21: LongInt;
  v22: Double;
  v23: Double;
  v24: Double;
  v25: Double;
begin
  v7 := -v5;
  v8 := MutCreate3(v7);
  while method4(v5, v8) do begin
      v10 := MutCreate3(v7);
      while method5(v5, v10) do begin
          v12 := v8.l0;
          v13 := v10.l0;
          v14 := 59;
          method6(v0, v1, v2, v3, v4, v6, v12, v13, v7, v14);
          v15 := 92;
          method6(v0, v1, v2, v3, v4, v6, v5, v13, v12, v15);
          v16 := -v12;
          v17 := 47;
          method6(v0, v1, v2, v3, v4, v6, v7, v13, v16, v17);
          v18 := 61;
          method6(v0, v1, v2, v3, v4, v6, v16, v13, v5, v18);
          v19 := -v13;
          v20 := 62;
          method6(v0, v1, v2, v3, v4, v6, v12, v7, v19, v20);
          v21 := 60;
          method6(v0, v1, v2, v3, v4, v6, v12, v5, v13, v21);
          v22 := v10.l0;
          v23 := v22 + 0.6;
          v10.l0 := v23;
      end;
      v24 := v8.l0;
      v25 := v24 + 0.6;
      v8.l0 := v25;
  end;
end;
function method7(v0: TMut0): Boolean;
var
  v1: LongInt;
  v2: Boolean;
begin
  v1 := v0.l0;
  v2 := v1 < 44;
  Result := v2;
end;
function method8(v0: TMut0): Boolean;
var
  v1: LongInt;
  v2: Boolean;
begin
  v1 := v0.l0;
  v2 := v1 < 160;
  Result := v2;
end;
function method2(v0: TArray0; v1: TArray1; v2: Double; v3: Double; v4: Double; v5: LongInt): LongInt;
var
  v6: TMut0;
  v8: LongInt;
  v9: LongInt;
  v10: Double;
  v11: Double;
  v12: Double;
  v13: Double;
  v14: Double;
  v15: Double;
  v16: TMut2;
  v17: TMut0;
  v19: LongInt;
  v20: TMut0;
  v22: LongInt;
  v23: LongInt;
  v24: LongInt;
  v25: LongInt;
  v26: Boolean;
  v44: AnsiString;
  v27: AnsiString;
  v28: Boolean;
  v29: AnsiString;
  v30: Boolean;
  v31: AnsiString;
  v32: Boolean;
  v33: AnsiString;
  v34: Boolean;
  v35: AnsiString;
  v36: Boolean;
  v37: AnsiString;
  v38: AnsiString;
  v45: LongInt;
  v46: LongInt;
  v47: LongInt;
  v48: LongInt;
  v49: LongInt;
  v50: LongInt;
  v51: LongInt;
begin
  v6 := MutCreate0(0);
  while method0(v6) do begin
      v8 := v6.l0;
      v0[v8] := 0.0;
      v1[v8] := 46;
      v9 := v8 + 1;
      v6.l0 := v9;
  end;
  v10 := 20.0;
  v11 := (-40.0);
  method3(v0, v1, v2, v3, v4, v10, v11);
  v12 := 10.0;
  v13 := 10.0;
  method3(v0, v1, v2, v3, v4, v12, v13);
  v14 := 5.0;
  v15 := 40.0;
  method3(v0, v1, v2, v3, v4, v14, v15);
  Write('u001b[H');
  v16 := MutCreate2(v5);
  v17 := MutCreate0(0);
  while method7(v17) do begin
      v19 := v17.l0;
      v20 := MutCreate0(0);
      while method8(v20) do begin
          v22 := v20.l0;
          v23 := v19 * 160;
          v24 := v22 + v23;
          v25 := v1[v24];
          v26 := v25 = 59;
          if v26 then begin
              v27 := ';';
              v44 := v27;
          end else begin
              v28 := v25 = 92;
              if v28 then begin
                  v29 := '\';
                  v44 := v29;
              end else begin
                  v30 := v25 = 47;
                  if v30 then begin
                      v31 := '/';
                      v44 := v31;
                  end else begin
                      v32 := v25 = 61;
                      if v32 then begin
                          v33 := '=';
                          v44 := v33;
                      end else begin
                          v34 := v25 = 62;
                          if v34 then begin
                              v35 := '>';
                              v44 := v35;
                          end else begin
                              v36 := v25 = 60;
                              if v36 then begin
                                  v37 := '<';
                                  v44 := v37;
                              end else begin
                                  v38 := '.';
                                  v44 := v38;
                              end;
                          end;
                      end;
                  end;
              end;
          end;
          Write(v44);
          v45 := v16.l0;
          v46 := v45 * 31;
          v47 := v46 + v25;
          v48 := v47 mod 1000003;
          v16.l0 := v48;
          v49 := v22 + 1;
          v20.l0 := v49;
      end;
      Write(#10);
      v50 := v19 + 1;
      v17.l0 := v50;
  end;
  v51 := v16.l0;
  Result := v51;
end;
function SpiralMain: LongInt;
var
  v0: TArray0;
  tmp1: TArray0;
  v1: TMut0;
  v3: LongInt;
  v4: LongInt;
  v5: TArray1;
  tmp6: TArray1;
  v6: TMut0;
  v8: LongInt;
  v9: LongInt;
  v10: TMut1;
  v11: TMut2;
  v12: TMut0;
  v14: LongInt;
  v15: Double;
  v16: Double;
  v17: Double;
  v18: LongInt;
  v19: LongInt;
  v20: Double;
  v21: Double;
  v22: Double;
  v23: Double;
  v24: Double;
  v25: Double;
  v26: LongInt;
  v27: LongInt;
begin
  tmp1 := nil;
  SetLength(tmp1, 7040);
  v0 := tmp1;
  v1 := MutCreate0(0);
  while method0(v1) do begin
      v3 := v1.l0;
      v0[v3] := 0.0;
      v4 := v3 + 1;
      v1.l0 := v4;
  end;
  tmp6 := nil;
  SetLength(tmp6, 7040);
  v5 := tmp6;
  v6 := MutCreate0(0);
  while method0(v6) do begin
      v8 := v6.l0;
      v5[v8] := 46;
      v9 := v8 + 1;
      v6.l0 := v9;
  end;
  v10 := MutCreate1(0.0, 0.0, 0.0);
  v11 := MutCreate2(0);
  v12 := MutCreate0(0);
  while method1(v12) do begin
      v14 := v12.l0;
      v15 := v10.l0;
      v16 := v10.l1;
      v17 := v10.l2;
      v18 := v11.l0;
      v19 := method2(v0, v5, v15, v16, v17, v18);
      v11.l0 := v19;
      v20 := v10.l0;
      v21 := v10.l1;
      v22 := v10.l2;
      v23 := v20 + 0.05;
      v24 := v21 + 0.05;
      v25 := v22 + 0.01;
      v10.l0 := v23;
      v10.l1 := v24;
      v10.l2 := v25;
      v26 := v14 + 1;
      v12.l0 := v26;
  end;
  v27 := v11.l0;
  Write('cube: ', 60, ' frames, checksum ', v27, #10);
  Result := 0;
end;
begin
  Halt(SpiralMain);
end.
